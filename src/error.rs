use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuError {
    #[error("Daemon error: {0}")]
    DaemonizeError(String),

    #[error("IO error: {0}")]
    IOError(String),

    #[error("Ser/De error: {0}")]
    SerDeError(String),

    #[error("Keystore error: {0}")]
    KeystoreError(String),
}

impl From<std::io::Error> for AuError {
    fn from(err: std::io::Error) -> Self {
        AuError::IOError(err.to_string())
    }
}

impl From<serde_json::Error> for AuError {
    fn from(err: serde_json::Error) -> Self {
        AuError::SerDeError(err.to_string())
    }
}

impl From<daemonize::DaemonizeError> for AuError {
    fn from(err: daemonize::DaemonizeError) -> Self {
        AuError::DaemonizeError(err.to_string())
    }
}

impl From<top_keystore_rs::KeystoreError> for AuError {
    fn from(err: top_keystore_rs::KeystoreError) -> Self {
        AuError::KeystoreError(err.to_string())
    }
}
