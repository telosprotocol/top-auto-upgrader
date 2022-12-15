use std::sync::{Arc, Mutex};

use tokio::time::{sleep, Duration};

use crate::{
    commands::{ProcessStatus, TopioCommands},
    config::ConfigJson,
    error::AuError,
    frequency::FrequencyControl,
    logic::LogicRunner,
};

pub struct KeepAliveLogic {
    config: Arc<ConfigJson>,
    frequency: Arc<Mutex<FrequencyControl>>,
}

impl KeepAliveLogic {
    pub fn new(config: Arc<ConfigJson>) -> Self {
        Self {
            config,
            frequency: Arc::new(Mutex::new(FrequencyControl::new(
                Duration::from_secs(1 * 60),
                Duration::from_secs(2 * 60),
                Duration::from_secs(1 * 60),
                Duration::from_secs(10 * 60),
            ))),
        }
    }

    fn inner_run(&self) -> Result<(), AuError> {
        let cmd = TopioCommands::new(
            self.config.user_config.user(),
            self.config.user_config.exec_dir(),
        );
        match (cmd.topio_status()?, cmd.safebox_status()?) {
            (ProcessStatus::NeedReset, _)
            | (_, ProcessStatus::NeedReset)
            | (ProcessStatus::Stoped, ProcessStatus::Stoped) => {
                // Need totally reset && restart
                if self.frequency.lock().unwrap().call_if_allowed() {
                    self.reset_safebox(&cmd)?;
                    self.restart_topio(&cmd)?;
                }
                Ok(())
            }
            (ProcessStatus::Stoped, ProcessStatus::Ok) => {
                // restart topio only
                if self.frequency.lock().unwrap().call_if_allowed() {
                    self.restart_topio(&cmd)?;
                }
                Ok(())
            }
            (ProcessStatus::Ok, _) => Ok(()),
        }
    }

    fn restart_topio(&self, cmd: &TopioCommands) -> Result<(), AuError> {
        println!("restart_topio");
        _ = cmd.start_topio()?;
        Ok(())
    }

    fn reset_safebox(&self, cmd: &TopioCommands) -> Result<(), AuError> {
        println!("reset_safebox");
        _ = cmd.kill_topio()?;
        _ = cmd.check_version()?;
        _ = cmd.set_miner_key(
            self.config.user_config.pubkey(),
            self.config.fetch_password(),
        );
        Ok(())
    }
}

impl LogicRunner for KeepAliveLogic {
    async fn loop_run(&self) {
        loop {
            {
                let r = self.inner_run();
                println!("{:?}", r);
            }
            sleep(Duration::from_secs(1)).await; // make it random
        }
    }
}
