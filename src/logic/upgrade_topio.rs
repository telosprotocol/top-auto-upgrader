use rand::Rng;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::time::{sleep, Duration};

use crate::{
    commands::TopioCommands,
    config::ConfigJson,
    error::AuError,
    frequency::FrequencyControl,
    version::{SemVersion, VersionHandler},
};

pub struct UpgradeTopioLogic {
    logic_mutex: Arc<Mutex<i32>>,
    config: Arc<ConfigJson>,
    frequency: Arc<Mutex<FrequencyControl>>,
}

impl UpgradeTopioLogic {
    pub async fn loop_run(&self) {
        let mut rng = rand::thread_rng();
        loop {
            {
                if let Ok(_) = self.logic_mutex.try_lock() {
                    let r = self.inner_run().await;
                    println!("UpgradeTopioLogic {:?}", r);
                }
            }
            sleep(Duration::from_secs(rng.gen_range(10..100))).await;
        }
    }
    pub fn new(logic_mutex: Arc<Mutex<i32>>, config: Arc<ConfigJson>) -> Self {
        Self {
            logic_mutex,
            config: config,
            frequency: Arc::new(Mutex::new(FrequencyControl::new(
                Duration::from_secs(0),
                Duration::from_secs(10 * 60),
                Duration::from_secs(10 * 60),
                Duration::from_secs(120 * 60),
            ))),
        }
    }

    async fn inner_run(&self) -> Result<(), AuError> {
        if self.frequency.lock().unwrap().call_if_allowed() {
            let latest_release = VersionHandler::new(
                self.config.au_config.api(),
                self.config.au_config.source_type(),
            )
            .get_release_info()
            .await?;
            if let Some(latest_version) = latest_release.version() {
                let cmd = TopioCommands::new(
                    self.config.user_config.user(),
                    self.config.user_config.exec_dir(),
                );
                let version_str = cmd.get_version()?;
                let current_version = SemVersion::from_str(&version_str)?;
                if latest_version.gt(&current_version) {
                    println!(
                        "try update from {} to {} ",
                        current_version.to_string(),
                        latest_version.to_string()
                    );
                    self.do_update(cmd, latest_version)?;
                }
            }
        }
        Ok(())
    }
    fn do_update(&self, cmd: TopioCommands, latest_version: SemVersion) -> Result<(), AuError> {
        _ = cmd.kill_topio()?;
        _ = cmd.wget_new_topio(latest_version.to_string())?;
        _ = cmd.install_new_topio(latest_version.to_string())?;
        _ = cmd.set_miner_key(
            self.config.user_config.pubkey(),
            self.config.fetch_password(),
        )?;
        _ = cmd.start_topio()?;
        Ok(())
    }
}
