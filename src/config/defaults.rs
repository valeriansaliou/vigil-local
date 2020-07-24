// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub fn server_log_level() -> String {
    "error".to_string()
}

pub fn metrics_poll_interval() -> u64 {
    60
}

pub fn metrics_poll_retry() -> u8 {
    2
}

pub fn metrics_poll_http_status_healthy_above() -> u16 {
    200
}

pub fn metrics_poll_http_status_healthy_below() -> u16 {
    400
}

pub fn metrics_poll_delay_dead() -> u64 {
    10
}

pub fn metrics_poll_delay_sick() -> u64 {
    1
}

pub fn metrics_script_interval() -> u64 {
    180
}
