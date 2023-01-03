use rand::Rng;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::time::{sleep, Duration};

use crate::{
    commands::{JoinStatus, TopioCommands},
    config::ConfigJson,
    error::AuError,
    frequency::FrequencyControl,
    version::{ReleaseInfo, SemVersion, VersionHandler},
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
        let interval_base = config.au_config.logic_frequency_base();
        Self {
            logic_mutex,
            config: config,
            frequency: Arc::new(Mutex::new(FrequencyControl::new(
                Duration::from_secs(0),
                Duration::from_secs(10 * interval_base),
                Duration::from_secs(10 * interval_base),
                Duration::from_secs(120 * interval_base),
            ))),
        }
    }

    async fn inner_run(&self) -> Result<(), AuError> {
        if !self.frequency.lock().unwrap().call_if_allowed() {
            return Ok(());
        }

        let latest_release = VersionHandler::new(
            self.config.au_config.api(),
            self.config.au_config.source_type(),
        )
        .get_release_info(None)
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

                if false == self.do_update(&cmd, latest_version, latest_release).await? {
                    println!(" revert back to {}", current_version.to_string());
                    let current_release = VersionHandler::new(
                        self.config.au_config.api(),
                        self.config.au_config.source_type(),
                    )
                    .get_release_info(Some(current_version.to_tag_name()))
                    .await?;
                    if false
                        == self
                            .do_update(&cmd, current_version, current_release)
                            .await?
                    {
                        return Err(AuError::CustomError(
                            "upgrader failed && revert to old version failed".into(),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    async fn do_update(
        &self,
        cmd: &TopioCommands,
        version_info: SemVersion,
        release_info: ReleaseInfo,
    ) -> Result<bool, AuError> {
        _ = cmd.kill_topio()?;
        let (asset_link, asset_name) = release_info
            .release_asset()
            .ok_or(AuError::CustomError("asset error".into()))?;
        _ = cmd.wget_new_topio(asset_link, asset_name)?;
        _ = cmd.install_new_topio(version_info.to_string())?;
        _ = cmd.set_miner_key(
            self.config.user_config.pubkey(),
            self.config.fetch_password(),
        )?;
        _ = cmd.start_topio()?;
        sleep(Duration::from_secs(5)).await;
        let mut wait_cnt = 0;
        loop {
            match cmd.check_is_joined()? {
                JoinStatus::NotReady => {
                    wait_cnt = wait_cnt + 1;
                    if wait_cnt >= 120 {
                        return Ok(false);
                    }
                    sleep(Duration::from_secs(5)).await;
                }
                JoinStatus::Yes => {
                    return Ok(true);
                }
                JoinStatus::NotRunning => {
                    return Ok(false);
                }
            }
        }
    }
}
