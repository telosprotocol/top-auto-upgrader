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

    #[error("Http error: {0}")]
    HttpError(String),

    #[error("Json error: {0}")]
    JsonParseError(String),

    #[error("std error: {0}")]
    StdError(String),

    #[error("custom error: {0}")]
    CustomError(String),
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

impl From<hyper::http::Error> for AuError {
    fn from(err: hyper::http::Error) -> Self {
        AuError::HttpError(err.to_string())
    }
}

impl From<hyper::Error> for AuError {
    fn from(err: hyper::Error) -> Self {
        AuError::HttpError(err.to_string())
    }
}

impl From<json::Error> for AuError {
    fn from(err: json::Error) -> Self {
        AuError::JsonParseError(err.to_string())
    }
}

impl From<std::str::Utf8Error> for AuError {
    fn from(err: std::str::Utf8Error) -> Self {
        AuError::StdError(err.to_string())
    }
}

impl From<std::num::ParseIntError> for AuError {
    fn from(err: std::num::ParseIntError) -> Self {
        AuError::StdError(err.to_string())
    }
}
