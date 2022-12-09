use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

mod user_config;
use user_config::UserConfigJson;

mod env_config;
use env_config::EnvConfigJson;

mod au_config;
use au_config::AuConfigJson;

mod temp_config;
use temp_config::TempConfigJson;

use crate::error::AuError;

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigJson {
    pub user_config: UserConfigJson,
    pub env_config: EnvConfigJson,
    pub au_config: AuConfigJson,
    pub temp_config: TempConfigJson,
}

impl ConfigJson {
    pub fn read_from_file(file_path_str: &str) -> Result<Self, AuError> {
        let file_path = Path::new(file_path_str);
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut config: Self = serde_json::from_str(&content)?;
        config.preprocess();
        Ok(config)
    }

    fn preprocess(&mut self) {
        self.try_encrypt_password();
    }

    fn try_encrypt_password(&mut self) {
        let pswd = self.temp_config.take_pswd();
        if !pswd.is_empty() {
            self.user_config.set_pswd(self.env_config.encrypt(pswd));
        }
    }

    pub fn fetch_password(&self) -> String {
        self.env_config.decrypt(self.user_config.get_enc_pswd())
    }
}
