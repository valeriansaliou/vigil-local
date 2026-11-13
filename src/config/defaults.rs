// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub fn server_log_level() -> String {
    "error".to_string()
}

pub fn report_error_retry_times() -> u8 {
    4
}

pub fn report_error_retry_after() -> u64 {
    2
}

pub fn metrics_interval() -> u64 {
    120
}

pub fn metrics_poll_retry() -> u8 {
    2
}

pub fn metrics_poll_retry_wait() -> u64 {
    200
}

pub fn metrics_poll_delay_dead() -> u64 {
    10
}

pub fn metrics_poll_delay_sick() -> u64 {
    1
}
