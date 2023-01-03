use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};

use crate::{
    commands::{ProcessStatus, TopioCommands},
    config::ConfigJson,
    error::AuError,
    frequency::FrequencyControl,
};

pub struct KeepAliveLogic {
    logic_mutex: Arc<Mutex<i32>>,
    config: Arc<ConfigJson>,
    frequency: Arc<Mutex<FrequencyControl>>,
}

impl KeepAliveLogic {
    pub async fn loop_run(&self) {
        let mut rng = rand::thread_rng();
        loop {
            {
                if let Ok(_) = self.logic_mutex.try_lock() {
                    let r = self.inner_run();
                    println!("KeepAliveLogic {:?}", r);
                }
            }
            sleep(Duration::from_secs(rng.gen_range(10..100))).await;
        }
    }
    pub fn new(logic_mutex: Arc<Mutex<i32>>, config: Arc<ConfigJson>) -> Self {
        let interval_base = config.au_config.logic_frequency_base();
        Self {
            logic_mutex,
            config,
            frequency: Arc::new(Mutex::new(FrequencyControl::new(
                Duration::from_secs(0), // instant exec at first
                Duration::from_secs(2 * interval_base),
                Duration::from_secs(2 * interval_base),
                Duration::from_secs(120 * interval_base),
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
            | (ProcessStatus::Stoped, ProcessStatus::Stoped)
            | (ProcessStatus::Stoped, ProcessStatus::Ok) => {
                // Need totally reset && restart
                // println!("stop && stop");
                if self.frequency.lock().unwrap().call_if_allowed() {
                    self.reset_safebox(&cmd)?;
                    self.restart_topio(&cmd)?;
                }
                Ok(())
            }
            // (ProcessStatus::Stoped, ProcessStatus::Ok) => {
            //     // restart topio only
            //     if self.frequency.lock().unwrap().call_if_allowed() {
            //         self.restart_topio(&cmd)?;
            //     }
            //     Ok(())
            // }
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
        _ = cmd.start_safebox()?;
        _ = cmd.set_miner_key(
            self.config.user_config.pubkey(),
            self.config.fetch_password(),
        )?;
        Ok(())
    }
}
