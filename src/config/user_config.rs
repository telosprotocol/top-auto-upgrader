use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserConfigJson {
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
}
