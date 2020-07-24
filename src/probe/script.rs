// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use super::report::{status as report_status, ReportReplica};
use super::status::Status;
use crate::config::config::{ConfigProbeService, ConfigProbeServiceNode};

pub fn dispatch(service: &ConfigProbeService, node: &ConfigProbeServiceNode, interval: u64) {
    if let Some(ref scripts) = node.scripts {
        if !scripts.is_empty() {
            debug!("script node has scripts in service node: #{}", node.id);

            for (index, script) in scripts.iter().enumerate() {
                // TODO
                let replica_status = Status::Dead;

                debug!("got replica status upon script: {:?}", replica_status);

                match report_status(
                    &service,
                    node,
                    ReportReplica::Script(&index.to_string()),
                    &replica_status,
                    interval,
                ) {
                    Ok(_) => info!("reported script replica status: {:?}", replica_status),
                    Err(_) => warn!(
                        "failed reporting script replica status: {:?}",
                        replica_status
                    ),
                }
            }

            return;
        }
    }

    warn!(
        "script node has no usable script in service node: #{}",
        node.id
    );
}
