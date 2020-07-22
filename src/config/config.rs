// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::defaults;
use super::regex::Regex;
use crate::probe::mode::Mode;
use crate::probe::replica::ReplicaURL;

#[derive(Deserialize)]
pub struct Config {
    pub server: ConfigServer,
    pub report: ConfigReport,
    pub metrics: ConfigMetrics,
    pub probe: ConfigProbe,
}

#[derive(Deserialize)]
pub struct ConfigServer {
    #[serde(default = "defaults::server_log_level")]
    pub log_level: String,
}

#[derive(Deserialize)]
pub struct ConfigReport {
    pub endpoint: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct ConfigMetrics {
    #[serde(default = "defaults::metrics_poll_interval")]
    pub poll_interval: u64,

    #[serde(default = "defaults::metrics_poll_retry")]
    pub poll_retry: u8,

    #[serde(default = "defaults::metrics_poll_http_status_healthy_above")]
    pub poll_http_status_healthy_above: u16,

    #[serde(default = "defaults::metrics_poll_http_status_healthy_below")]
    pub poll_http_status_healthy_below: u16,

    #[serde(default = "defaults::metrics_poll_delay_dead")]
    pub poll_delay_dead: u64,

    #[serde(default = "defaults::metrics_poll_delay_sick")]
    pub poll_delay_sick: u64,

    #[serde(default = "defaults::metrics_script_interval")]
    pub script_interval: u64,
}

#[derive(Deserialize)]
pub struct ConfigProbe {
    pub service: Vec<ConfigProbeService>,
}

#[derive(Deserialize)]
pub struct ConfigProbeService {
    pub id: String,
    pub node: Vec<ConfigProbeServiceNode>,
}

#[derive(Deserialize)]
pub struct ConfigProbeServiceNode {
    pub id: String,
    pub mode: Mode,
    pub replicas: Option<Vec<ReplicaURL>>,
    pub scripts: Option<Vec<String>>,
    pub http_body_healthy_match: Option<Regex>,
}
