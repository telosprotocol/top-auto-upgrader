#![feature(never_type)]

mod commands;
mod config;
mod error;
mod version;

use clap::Parser;
use daemonize::Daemonize;
use error::AuError;
use tokio::time::{sleep, Duration};

use crate::config::ConfigJson;

fn test_run() -> ! {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            loop {
                sleep(Duration::from_secs(10)).await;
            }
        })
}

#[derive(Parser)]
struct AuArgs {
    /// daemon
    #[clap(short = 'd', long = "daemon")]
    daemon: bool,

    /// config file
    #[clap(short = 'c', long = "config")]
    config: String,

    /// check config file only.
    #[clap(long = "check")]
    check: bool,
}

fn main() -> Result<(), AuError> {
    let args = AuArgs::parse();

    if args.check {
        ConfigJson::check_config_file(&args.config)?;
        return Ok(());
    }

    let config_json = ConfigJson::read_from_file(&args.config)?;

    // println!("config_Json: {:?}", config_json);

    // let r = config_json.fetch_password();

    // println!("password: {:?}", r);

    if args.daemon {
        let daemonize = Daemonize::new();
        daemonize.start()?;
        println!("daemon start ok"); // won't see
    }

    println!("Top Auto Upgrader Start!");

    test_run()
}
