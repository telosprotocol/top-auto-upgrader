use serde::{Deserialize, Serialize};

use crate::{commands::read_file, error::AuError};

use top_keystore_rs::{decrypt_T0_keystore_file, decrypt_T8_keystore_file};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigJson {
    mining_keystore_file_dir: String,
    mining_pub_key: String,
    mining_pswd_enc: String,
    topio_package_dir: String,
    topio_user: String,
}

impl UserConfigJson {
    pub(crate) fn set_pswd(&mut self, enc_pswd: String) {
        self.mining_pswd_enc = enc_pswd;
    }

    pub(crate) fn get_enc_pswd(&self) -> &str {
        &self.mining_pswd_enc
    }

    pub(crate) fn try_decrypt_keystore(&self, pswd: String) -> Result<(), AuError> {
        let keystore_file_content = read_file(&self.mining_keystore_file_dir)?;

        _ = decrypt_T0_keystore_file(keystore_file_content.clone(), pswd.clone())
            .or_else(|_| decrypt_T8_keystore_file(keystore_file_content.clone(), pswd.clone()))?;
        Ok(())
    }
}
