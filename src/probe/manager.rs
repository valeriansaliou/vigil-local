// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use crate::probe::mode::Mode;
use crate::APP_CONF;
use std::thread;
use std::time::Duration;

use super::poll::dispatch as poll_dispatch;
use super::script::dispatch as script_dispatch;

const PROBE_RUN_HOLD_SECONDS: u64 = 2;
const PROBE_CHECK_INTERVAL_SECONDS: u64 = 120;

pub fn run() {
    // Hold on a bit before first cycle
    thread::sleep(Duration::from_secs(PROBE_RUN_HOLD_SECONDS));

    debug!("will run first probe cycle");

    // Start cycling
    loop {
        cycle();

        info!(
            "done cycling probe, holding for next cycle: {}s",
            PROBE_CHECK_INTERVAL_SECONDS
        );

        // Hold on a bit for next cycle
        thread::sleep(Duration::from_secs(PROBE_CHECK_INTERVAL_SECONDS));

        debug!("holding for next probe cycle, will run next cycle");
    }
}

fn cycle() {
    debug!("cycling through all services");

    for service in &APP_CONF.probe.service {
        debug!("scanning for nodes in service: #{}", service.id);

        for node in &service.node {
            debug!("scanning for targets in service node: #{}", node.id);

            match node.mode {
                Mode::Poll => poll_dispatch(service, node, PROBE_CHECK_INTERVAL_SECONDS),
                Mode::Script => script_dispatch(service, node, PROBE_CHECK_INTERVAL_SECONDS),
            }
        }
    }

    info!("done cycling through all services");
}
