use serde::{Deserialize, Serialize};

mod user_config;
use user_config::UserConfigJson;

mod env_config;
use env_config::EnvConfigJson;

mod au_config;
use au_config::AuConfigJson;

mod temp_config;
use temp_config::TempConfigJson;

use crate::{
    commands::{read_file, write_file},
    error::AuError,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigJson {
    pub user_config: UserConfigJson,
    pub env_config: EnvConfigJson,
    pub au_config: AuConfigJson,
    pub temp_config: TempConfigJson,

    // Never serialized.
    #[serde(skip)]
    config_path: String,
}

impl ConfigJson {
    pub fn read_from_file(file_path_str: &str) -> Result<Self, AuError> {
        let content = read_file(file_path_str)?;
        let mut config: Self = serde_json::from_str(&content)?;
        config.config_path = String::from(file_path_str); // save for furture use.
        Ok(config)
    }

    pub fn check_config_file(file_path_str: &str) -> Result<(), AuError> {
        let content = read_file(file_path_str)?;
        let mut config: Self = serde_json::from_str(&content)?;
        config.config_path = String::from(file_path_str); // save for furture use.

        // TODO check keystore
        config.try_decrypt_keystore()?;
        config.try_encrypt_password();
        config.update_config_file()?;
        Ok(())
    }

    pub fn update_config_file(&self) -> Result<(), AuError> {
        write_file(&self.config_path, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }

    fn try_encrypt_password(&mut self) {
        let pswd = self.temp_config.take_pswd();
        if !pswd.is_empty() {
            self.user_config.set_pswd(self.env_config.encrypt(pswd));
        }
    }

    fn try_decrypt_keystore(&mut self) -> Result<(), AuError> {
        let mut pswd = self.temp_config.take_pswd();
        if pswd.is_empty() {
            pswd = self.fetch_password();
        }
        self.user_config.try_decrypt_keystore(pswd)
    }

    pub fn fetch_password(&self) -> String {
        self.env_config.decrypt(self.user_config.get_enc_pswd())
    }
}
