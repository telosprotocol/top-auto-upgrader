// #![feature(never_type)]
enum NeverType {} // stable rust compromise

mod commands;
mod config;
mod error;
mod frequency;
mod logic;
mod version;

use std::sync::{Arc, Mutex};

use clap::Parser;
use daemonize::Daemonize;
use error::AuError;
use logic::{KeepAliveLogic, UpgradeTopioLogic};
use tokio::{
    join,
    time::{sleep, Duration},
};

use crate::config::ConfigJson;

fn test_run(config: ConfigJson) -> NeverType {
    let config = Arc::new(config);
    let logic_mutex = Arc::new(Mutex::new(0));
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let t = KeepAliveLogic::new(logic_mutex.clone(), config.clone());
            let k = UpgradeTopioLogic::new(logic_mutex.clone(), config.clone());
            join!(t.loop_run(), k.loop_run()); // won't exist.
            panic!("ERROR");
            #[allow(unreachable_code)]
            loop {
                sleep(Duration::from_secs(1)).await;
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

    test_run(config_json);
    #[allow(unreachable_code)]
    Ok(())
}
