// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::fmt;

use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use url::{Host, Url};

#[derive(Serialize, Debug, Clone)]
pub enum ReplicaURL {
    ICMP(String, String),
    TCP(String, String, u16),
    HTTP(String, String),
    HTTPS(String, String),
}

impl ReplicaURL {
    pub fn parse_from(raw_url: &str) -> Result<ReplicaURL, ()> {
        match Url::parse(raw_url) {
            Ok(url) => match url.scheme() {
                "icmp" => match url.host() {
                    Some(host) => Ok(ReplicaURL::ICMP(
                        raw_url.to_owned(),
                        Self::host_string(host),
                    )),
                    _ => Err(()),
                },
                "tcp" => match (url.host(), url.port()) {
                    (Some(host), Some(port)) => Ok(ReplicaURL::TCP(
                        raw_url.to_owned(),
                        Self::host_string(host),
                        port,
                    )),
                    _ => Err(()),
                },
                "http" => Ok(ReplicaURL::HTTP(raw_url.to_owned(), url.to_string())),
                "https" => Ok(ReplicaURL::HTTPS(raw_url.to_owned(), url.to_string())),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    pub fn get_raw(&self) -> &str {
        match self {
            &ReplicaURL::ICMP(ref raw_url, _) => raw_url,
            &ReplicaURL::TCP(ref raw_url, _, _) => raw_url,
            &ReplicaURL::HTTP(ref raw_url, _) => raw_url,
            &ReplicaURL::HTTPS(ref raw_url, _) => raw_url,
        }
    }

    fn host_string(host: Host<&str>) -> String {
        // Convert internal host value into string. This is especially useful for IPv6 addresses, \
        //   which we need returned in '::1' format; as they would otherwise be returned in \
        //   '[::1]' format using built-in top-level 'to_string()' method on the 'Host' trait. The \
        //   underlying address parser does not accept IPv6 addresses formatted as '[::1]', so \
        //   this seemingly overkill processing is obviously needed.
        match host {
            Host::Domain(domain_value) => domain_value.to_string(),
            Host::Ipv4(ipv4_value) => ipv4_value.to_string(),
            Host::Ipv6(ipv6_value) => ipv6_value.to_string(),
        }
    }
}

impl<'de> Deserialize<'de> for ReplicaURL {
    fn deserialize<D>(de: D) -> Result<ReplicaURL, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ReplicaURLVisitor;

        impl<'de> Visitor<'de> for ReplicaURLVisitor {
            type Value = ReplicaURL;

            fn expecting(&self, format: &mut fmt::Formatter) -> fmt::Result {
                format.write_str("an ICMP, TCP, HTTP or HTTPS url")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<ReplicaURL, E> {
                ReplicaURL::parse_from(value).map_err(|_| E::custom("invalid"))
            }
        }

        de.deserialize_str(ReplicaURLVisitor)
    }
}
