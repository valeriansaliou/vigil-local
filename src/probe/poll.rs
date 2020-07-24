// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use ping::ping;

use std::cmp::min;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::str::FromStr;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

use super::replica::ReplicaURL;
use super::report::{status as report_status, ReportReplica};
use super::status::Status;
use crate::config::config::{ConfigProbeService, ConfigProbeServiceNode};
use crate::APP_CONF;

const NODE_ICMP_TIMEOUT_MILLISECONDS: u64 = 1000;
const RETRY_REPLICA_AFTER_MILLISECONDS: u64 = 200;

lazy_static! {
    static ref POLL_HTTP_HEADER_USERAGENT: String =
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}

pub fn dispatch(service: &ConfigProbeService, node: &ConfigProbeServiceNode, interval: u64) {
    if let Some(ref replicas) = node.replicas {
        if !replicas.is_empty() {
            debug!("poll node has replicas in service node: #{}", node.id);

            for replica in replicas {
                let replica_status = proceed_replica(&service.id, &node.id, replica);

                debug!("got replica status upon poll: {:?}", replica_status);

                match report_status(
                    &service,
                    node,
                    ReportReplica::Poll(replica),
                    &replica_status,
                    interval,
                ) {
                    Ok(_) => info!("reported poll replica status: {:?}", replica_status),
                    Err(_) => warn!("failed reporting poll replica status: {:?}", replica_status),
                }
            }

            return;
        }
    }

    warn!(
        "poll node has no usable replica in service node: #{}",
        node.id
    );
}

pub fn proceed_replica(service_id: &str, node_id: &str, replica: &ReplicaURL) -> Status {
    // Attempt to acquire (first attempt)
    proceed_replica_attempt(service_id, node_id, replica, APP_CONF.metrics.poll_retry, 0)
}

fn proceed_replica_attempt(
    service_id: &str,
    node_id: &str,
    replica: &ReplicaURL,
    retry_times: u8,
    attempt: u8,
) -> Status {
    info!(
        "running poll replica scan attempt #{} on #{}:#{}:[{:?}]",
        attempt, service_id, node_id, replica
    );

    match proceed_replica_request(service_id, node_id, replica) {
        Status::Healthy => Status::Healthy,
        Status::Sick => Status::Sick,
        Status::Dead => {
            let next_attempt = attempt + 1;

            if next_attempt > retry_times {
                Status::Dead
            } else {
                warn!(
                    "poll replica scan attempt #{} failed on #{}:#{}:[{:?}], will retry",
                    attempt, service_id, node_id, replica
                );

                // Retry after delay
                thread::sleep(Duration::from_millis(RETRY_REPLICA_AFTER_MILLISECONDS));

                proceed_replica_attempt(service_id, node_id, replica, retry_times, next_attempt)
            }
        }
    }
}

fn proceed_replica_request(service_id: &str, node_id: &str, replica: &ReplicaURL) -> Status {
    debug!(
        "scanning poll replica: #{}:#{}:[{:?}]",
        service_id, node_id, replica
    );

    let start_time = SystemTime::now();

    let (is_up, poll_duration) = match replica {
        &ReplicaURL::ICMP(_, ref host) => proceed_replica_request_icmp(host),
        &ReplicaURL::TCP(_, ref host, port) => proceed_replica_request_tcp(host, port),
        &ReplicaURL::HTTP(_, ref url) => proceed_replica_request_http(url),
        &ReplicaURL::HTTPS(_, ref url) => proceed_replica_request_http(url),
    };

    if is_up == true {
        // Acquire poll duration latency
        let duration_latency = match poll_duration {
            Some(poll_duration) => poll_duration,
            None => SystemTime::now()
                .duration_since(start_time)
                .unwrap_or(Duration::from_secs(0)),
        };

        if duration_latency >= Duration::from_secs(APP_CONF.metrics.poll_delay_sick) {
            Status::Sick
        } else {
            Status::Healthy
        }
    } else {
        Status::Dead
    }
}

fn proceed_replica_request_icmp(host: &str) -> (bool, Option<Duration>) {
    // Notice: a dummy port of value '0' is set here, so that we can resolve the host to an actual \
    //   IP address using the standard library, which avoids depending on an additional library.
    let address_results = (host, 0).to_socket_addrs();

    // Storage variable for the maximum round-trip-time found for received ping responses
    let mut maximum_rtt = None;

    match address_results {
        Ok(address) => {
            // Notice: the ICMP probe checker is a bit special, in the sense that it checks all \
            //   resolved addresses. As we check for an host health at the IP level (ie. not at \
            //   the application layer level), checking only the first host in the list is not \
            //   sufficient for the whole replica group to be up. This can be used as an handy way \
            //   to check for the health of a group of IP hosts, configured in a single DNS record.
            let address_values: Vec<SocketAddr> = address.collect();

            if !address_values.is_empty() {
                debug!(
                    "prober poll will fire for icmp host: {} ({} targets)",
                    host,
                    address_values.len()
                );

                // As ICMP pings require a lower-than-usual timeout, an hard-coded ICMP \
                //   timeout value is used by default, though the configured dead delay value \
                //   is preferred in the event it is lower than the hard-coded value (unlikely \
                //   though possible in some setups).
                let pinger_timeout = Duration::from_secs(min(
                    NODE_ICMP_TIMEOUT_MILLISECONDS,
                    acquire_dead_timeout().as_secs() * 1000,
                ));

                // Probe all returned addresses (sequentially)
                for address_value in &address_values {
                    let address_ip = address_value.ip();

                    debug!(
                        "prober poll will send icmp ping to target: {} from host: {}",
                        address_ip, host
                    );

                    // Acquire ping start time (used for RTT calculation)
                    let ping_start_time = SystemTime::now();

                    // Ping target IP address
                    match ping(address_ip, Some(pinger_timeout), None, None, None, None) {
                        Ok(_) => {
                            debug!(
                                "got prober poll response for icmp target: {} from host: {}",
                                address_ip, host
                            );

                            // Process ping RTT
                            let ping_rtt = SystemTime::now()
                                .duration_since(ping_start_time)
                                .unwrap_or(Duration::from_secs(0));

                            // Do not return (consider address as reachable)
                            // Notice: update maximum observed round-trip-time, if higher than \
                            //   last highest observed.
                            maximum_rtt = match maximum_rtt {
                                Some(maximum_rtt) => {
                                    if ping_rtt > maximum_rtt {
                                        Some(ping_rtt)
                                    } else {
                                        Some(maximum_rtt)
                                    }
                                }
                                None => Some(ping_rtt),
                            };
                        }
                        Err(err) => {
                            debug!(
                                "prober poll error for icmp target: {} from host: {} (error: {})",
                                address_ip, host, err
                            );

                            // Consider ICMP errors as a failure
                            return (false, None);
                        }
                    }
                }
            } else {
                debug!(
                    "prober poll did not resolve any address for icmp replica: {}",
                    host
                );

                // Consider empty as a failure
                return (false, None);
            }
        }
        Err(err) => {
            error!(
                "prober poll address for icmp replica is invalid: {} (error: {})",
                host, err
            );

            // Consider invalid URL as a failure
            return (false, None);
        }
    };

    // If there was no early return, consider all the hosts as reachable for replica
    (true, maximum_rtt)
}

fn proceed_replica_request_tcp(host: &str, port: u16) -> (bool, Option<Duration>) {
    let address_results = (host, port).to_socket_addrs();

    if let Ok(mut address) = address_results {
        if let Some(address_value) = address.next() {
            debug!("prober poll will fire for tcp target: {}", address_value);

            return match TcpStream::connect_timeout(&address_value, acquire_dead_timeout()) {
                Ok(_) => (true, None),
                Err(_) => (false, None),
            };
        }
    }

    (false, None)
}

fn proceed_replica_request_http(url: &str) -> (bool, Option<Duration>) {
    debug!("prober poll will fire for http target: {}", &url);

    // Unpack dead timeout
    let dead_timeout = acquire_dead_timeout();

    // Acquire replica response
    let mut response_body = Vec::new();

    let response = Request::new(&Uri::from_str(&url).expect("invalid replica request uri"))
        .connect_timeout(Some(dead_timeout))
        .read_timeout(Some(dead_timeout))
        .write_timeout(Some(dead_timeout))
        .method(Method::HEAD)
        .header("User-Agent", &*POLL_HTTP_HEADER_USERAGENT)
        .send(&mut response_body);

    // Handle response
    if let Ok(response) = response {
        let status_code = u16::from(response.status_code());

        debug!(
            "prober poll result received for url: {} with status: {}",
            &url, status_code
        );

        // Consider as UP?
        if status_code >= APP_CONF.metrics.poll_http_status_healthy_above
            && status_code < APP_CONF.metrics.poll_http_status_healthy_below
        {
            return (true, None);
        }
    } else {
        debug!("prober poll result was not received for url: {}", &url);
    }

    // Consider as DOWN.
    (false, None)
}

fn acquire_dead_timeout() -> Duration {
    Duration::from_secs(APP_CONF.metrics.poll_delay_dead)
}
