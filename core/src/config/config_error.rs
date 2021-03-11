use thiserror::Error;

use crate::file::FileError;

use serde_yaml;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("file: {source}")]
    File {
        #[from]
        source: FileError,
    },
    #[error("yaml: {source}")]
    Yaml {
        #[from]
        source: serde_yaml::Error,
    },
    #[error("unknown key: {0}")]
    UnknownKey(String),
    #[error("invalid value type: key = {0}, requested type = {1}")]
    InvalidValueType(String, String),
}
