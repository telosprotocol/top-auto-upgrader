use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TempConfigJson {
    temp_pswd: String,
}

impl TempConfigJson {
    pub(crate) fn take_pswd(&mut self) -> String {
        self.temp_pswd.drain(..).collect()
    }
}
