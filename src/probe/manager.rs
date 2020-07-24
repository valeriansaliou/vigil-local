// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use std::thread;
use std::time::Duration;

use super::poll::dispatch as poll_dispatch;
use super::script::dispatch as script_dispatch;
use crate::probe::mode::Mode;
use crate::APP_CONF;

const PROBE_RUN_HOLD_SECONDS: u64 = 2;

pub fn run() {
    // Hold on a bit before first cycle
    thread::sleep(Duration::from_secs(PROBE_RUN_HOLD_SECONDS));

    debug!("will run first probe cycle");

    // Start cycling
    loop {
        cycle();

        info!(
            "done cycling probe, holding for next cycle: {}s",
            APP_CONF.metrics.interval
        );

        // Hold for next aggregate run
        thread::sleep(Duration::from_secs(APP_CONF.metrics.interval));

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
                Mode::Poll => poll_dispatch(service, node, APP_CONF.metrics.interval),
                Mode::Script => script_dispatch(service, node, APP_CONF.metrics.interval),
            }
        }
    }

    info!("done cycling through all services");
}
