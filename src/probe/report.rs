// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use base64::engine::general_purpose::STANDARD as base64_encoder;
use base64::Engine;
use http_req::{
    request::{Method, Request},
    uri::Uri,
};
use serde_json;

use std::convert::TryFrom;
use std::io;
use std::thread;
use std::time::Duration;

use super::replica::ReplicaURL;
use super::status::Status;
use crate::config::config::{ConfigProbeService, ConfigProbeServiceNode};
use crate::APP_CONF;

pub const REPORT_HTTP_CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

const RETRY_STATUS_TIMES: u8 = 4;
const RETRY_STATUS_AFTER_SECONDS: u64 = 2;

#[derive(Debug, Clone, Copy)]
pub enum ReportReplica<'a> {
    Poll(&'a ReplicaURL, Option<&'a str>),
    Script(&'a str, Option<&'a str>),
}

#[derive(Serialize)]
struct ReportPayload<'a> {
    replica: &'a str,
    health: &'a str,
    interval: u64,
}

lazy_static! {
    pub static ref REPORT_HTTP_HEADER_USERAGENT: String =
        format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    pub static ref REPORT_HTTP_HEADER_AUTHORIZATION: String = format!(
        "Basic {}",
        base64_encoder.encode(&format!(":{}", APP_CONF.report.token))
    );
}

impl<'a> ReportReplica<'a> {
    pub fn to_string(&self) -> String {
        let values = match self {
            Self::Poll(replica, prefix) => (replica.get_raw(), prefix),
            Self::Script(replica, prefix) => (*replica, prefix),
        };

        match values {
            (replica, Some(prefix)) => format!("{}:{}", prefix, replica),
            (replica, None) => replica.to_string(),
        }
    }
}

pub fn generate_url(path: &str) -> String {
    format!("{}/{}", &APP_CONF.report.endpoint, path)
}

pub fn status<'a>(
    service: &ConfigProbeService,
    node: &ConfigProbeServiceNode,
    replica: ReportReplica<'a>,
    status: &Status,
    interval: u64,
) -> Result<(), ()> {
    // Attempt to acquire (first attempt)
    status_attempt(service, node, replica, status, interval, 0)
}

fn status_attempt<'a>(
    service: &ConfigProbeService,
    node: &ConfigProbeServiceNode,
    replica: ReportReplica<'a>,
    status: &Status,
    interval: u64,
    attempt: u8,
) -> Result<(), ()> {
    info!(
        "running status report attempt #{} on #{}:#{}:[{:?}]",
        attempt, service.id, node.id, replica
    );

    match status_request(service, node, replica, status, interval) {
        Ok(_) => Ok(()),
        Err(_) => {
            let next_attempt = attempt + 1;

            if next_attempt >= RETRY_STATUS_TIMES {
                Err(())
            } else {
                error!(
                    "status report attempt #{} failed on #{}:#{}:[{:?}], will retry",
                    attempt, service.id, node.id, replica
                );

                // Retry after delay
                thread::sleep(Duration::from_secs(RETRY_STATUS_AFTER_SECONDS));

                status_attempt(service, node, replica, status, interval, next_attempt)
            }
        }
    }
}

fn status_request<'a>(
    service: &ConfigProbeService,
    node: &ConfigProbeServiceNode,
    replica: ReportReplica<'a>,
    status: &Status,
    interval: u64,
) -> Result<(), ()> {
    // Generate report URL and replica ID
    let (report_url, replica_id) = (
        generate_url(&format!("reporter/{}/{}/", &service.id, &node.id)),
        replica.to_string(),
    );

    debug!(
        "generated report url: {} for replica: {}",
        &report_url, &replica_id
    );

    // Generate report payload
    let payload = ReportPayload {
        replica: &replica_id,
        interval: interval,
        health: status.as_str(),
    };

    // Encode payload to string
    // Notice: fail hard if payload is invalid (it should never be)
    let payload_json = serde_json::to_vec(&payload).expect("invalid status request payload");

    // Generate request URI
    let request_uri = Uri::try_from(report_url.as_str()).expect("invalid status request uri");

    // Acquire report response
    let mut response_sink = io::sink();

    let response = Request::new(&request_uri)
        .connect_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .read_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .write_timeout(Some(REPORT_HTTP_CLIENT_TIMEOUT))
        .method(Method::POST)
        .header("User-Agent", &*REPORT_HTTP_HEADER_USERAGENT)
        .header("Authorization", &*REPORT_HTTP_HEADER_AUTHORIZATION)
        .header("Content-Type", "application/json")
        .header("Content-Length", &payload_json.len())
        .body(&payload_json)
        .send(&mut response_sink);

    match response {
        Ok(response) => {
            let status_code = response.status_code();

            if status_code.is_success() {
                debug!("reported to probe url: {}", report_url);

                Ok(())
            } else {
                debug!(
                    "could not report to probe url: {} (got status code: {})",
                    report_url, status_code
                );

                Err(())
            }
        }
        Err(err) => {
            warn!(
                "failed reporting to probe url: {} because: {}",
                report_url, err
            );

            Err(())
        }
    }
}
