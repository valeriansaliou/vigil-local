// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::report::status as report_status;
use super::status::Status;
use crate::APP_CONF;

pub fn dispatch(interval: u64) {
    // TODO: commonize w/ the poll reporter (loop should be the same)
    debug!("will dispatch scripts");

    for service in &APP_CONF.probe.service {
        debug!("scanning for scripts in service: #{}", service.id);

        for node in &service.node {
            debug!("scanning for scripts in service node: #{}", node.id);

            if let Some(ref scripts) = node.scripts {
                for script in scripts {
                    // TODO
                    let replica_status = Status::Dead;

                    debug!("got replica status upon script: {:?}", replica_status);

                    // TODO: move replica to an enum container
                    // match report_status(&service, node, &replica, &replica_status, interval) {
                    //     Ok(_) => info!("reported script replica status: {:?}", replica_status),
                    //     Err(_) => warn!(
                    //         "failed reporting script replica status: {:?}",
                    //         replica_status
                    //     ),
                    // }
                }
            }
        }
    }

    info!("dispatched scripts");
}
