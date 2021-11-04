// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use run_script::{self, ScriptOptions};

use super::report::{status as report_status, ReportReplica};
use super::status::Status;
use crate::config::config::{ConfigProbeService, ConfigProbeServiceNode};

pub fn dispatch(service: &ConfigProbeService, node: &ConfigProbeServiceNode, interval: u64) {
    if let Some(ref scripts) = node.scripts {
        if !scripts.is_empty() {
            debug!("script node has scripts in service node: #{}", node.id);

            for (index, script) in scripts.iter().enumerate() {
                let replica_id = index.to_string();
                let replica_status = proceed_replica(&service.id, &node.id, &replica_id, script);

                debug!("got replica status upon script: {:?}", replica_status);

                match report_status(
                    &service,
                    node,
                    ReportReplica::Script(&replica_id),
                    &replica_status,
                    interval,
                ) {
                    Ok(_) => info!("reported script replica status: {:?}", replica_status),
                    Err(_) => error!(
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

pub fn proceed_replica(service_id: &str, node_id: &str, replica_id: &str, script: &str) -> Status {
    info!(
        "executing script replica on #{}:#{}:[#{}]",
        service_id, node_id, replica_id
    );

    match run_script::run(script, &Vec::new(), &ScriptOptions::new()) {
        Ok((code, _, _)) => {
            // Return code '0' goes for 'healthy', '1' goes for 'sick'; any other code is 'dead'
            let replica_status = match code {
                0 => Status::Healthy,
                1 => Status::Sick,
                _ => Status::Dead,
            };

            if replica_status == Status::Dead {
                warn!(
                    "script replica execution succeeded with {:?} return code: {}",
                    replica_status, code
                );
            } else {
                debug!(
                    "script replica execution succeeded with {:?} return code: {}",
                    replica_status, code
                );
            }

            replica_status
        }
        Err(err) => {
            error!("script replica execution failed with error: {}", err);

            Status::Dead
        }
    }
}
