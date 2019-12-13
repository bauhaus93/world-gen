use std::fmt;
use std::error::Error;

use crate::file::FileError;

use serde_yaml;

#[derive(Debug)]
pub enum ConfigError {
    File(FileError),
    Yaml(serde_yaml::Error),
    UnknownKey(String),
    InvalidValueType(String, String)
}

impl From<FileError> for ConfigError {
    fn from(err: FileError) -> ConfigError {
        ConfigError::File(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> ConfigError {
        ConfigError::Yaml(err)
    }
}

impl Error for ConfigError {

    fn description(&self) -> &str {
        match *self {
            ConfigError::File(_) => "file",
            ConfigError::Yaml(_) => "yaml",
            ConfigError::UnknownKey(_) => "unknown key",
            ConfigError::InvalidValueType(_, _) => "invalid value type"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ConfigError::File(ref err) => Some(err),
            ConfigError::Yaml(ref err) => Some(err),
            ConfigError::UnknownKey(_) => None,
            ConfigError::InvalidValueType(_, _) => None
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::File(ref err) => write!(f, "{}: {}", self.description(), err),
            ConfigError::Yaml(ref err) => write!(f, "{}: {}", self.description(), err),
            ConfigError::UnknownKey(ref unknown_key) => write!(f, "{}: '{}'", self.description(), unknown_key),
            ConfigError::InvalidValueType(ref key, ref requested_type) => write!(f, "{}: key = '{}', requested type was '{}', but stored value isn't of that type",
                self.description(), key, requested_type)
        }
    }
}
