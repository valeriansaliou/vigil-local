// Vigil Local
//
// Vigil local probe relay
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

mod config;
mod probe;

use std::ops::Deref;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use clap::{Arg, Command};
use log::LevelFilter;

use config::config::Config;
use config::logger::ConfigLogger;
use config::reader::ConfigReader;
use probe::manager::run as run_probe;

struct AppArgs {
    config: String,
}

pub static THREAD_NAME_PROBE: &'static str = "vigil-local-probe";

lazy_static! {
    static ref APP_ARGS: AppArgs = make_app_args();
    static ref APP_CONF: Config = ConfigReader::make();
}

fn make_app_args() -> AppArgs {
    let matches = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Path to configuration file")
                .default_value("./config.cfg"),
        )
        .get_matches();

    // Generate owned app arguments
    AppArgs {
        config: matches
            .get_one::<String>("config")
            .expect("invalid config value")
            .to_owned(),
    }
}

fn ensure_states() {
    // Ensure all statics are valid (a `deref` is enough to lazily initialize them)
    let (_, _) = (APP_ARGS.deref(), APP_CONF.deref());
}

fn spawn_probe() {
    debug!("spawn managed thread: probe");

    let worker = thread::Builder::new()
        .name(THREAD_NAME_PROBE.to_string())
        .spawn(run_probe);

    // Block on worker thread (join it)
    let has_error = if let Ok(worker_thread) = worker {
        worker_thread.join().is_err()
    } else {
        true
    };

    // Worker thread crashed?
    if has_error == true {
        error!("managed thread crashed (probe), setting it up again");

        // Prevents thread start loop floods
        // Notice: 5 seconds here to prevent network floods
        thread::sleep(Duration::from_secs(5));

        spawn_probe();
    }
}

fn main() {
    // Initialize shared logger
    let _logger = ConfigLogger::init(
        LevelFilter::from_str(&APP_CONF.server.log_level).expect("invalid log level"),
    );

    info!("starting up");

    // Ensure all states are bound
    ensure_states();

    // Spawn probe (foreground thread)
    spawn_probe();

    error!("could not start");
}
