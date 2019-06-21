use std::fmt;
use std::error::Error;
use std::num;

use crate::FileError;
use super::Value;

#[derive(Debug)]
pub enum ConfigError {
    File(FileError),
    ParseFloat(num::ParseFloatError),
    ParseInt(num::ParseIntError),
    InvalidFieldCount(usize, usize, String),
    InvalidFieldType(String, String),
    UnknownKey(String),
    InvalidValueType(String, Value, Value)
}

impl From<FileError> for ConfigError {
    fn from(err: FileError) -> ConfigError {
        ConfigError::File(err)
    }
}

impl From<num::ParseFloatError> for ConfigError {
    fn from(err: num::ParseFloatError) -> ConfigError {
        ConfigError::ParseFloat(err)
    }
}

impl From<num::ParseIntError> for ConfigError {
    fn from(err: num::ParseIntError) -> ConfigError {
        ConfigError::ParseInt(err)
    }
}

impl Error for ConfigError {

    fn description(&self) -> &str {
        match *self {
            ConfigError::File(_) => "file",
            ConfigError::ParseFloat(_) => "parse float",
            ConfigError::ParseInt(_) => "parse int",
            ConfigError::InvalidFieldCount(_, _, _) => "invalid field count",
            ConfigError::InvalidFieldType(_, _) => "invalid field type",
            ConfigError::UnknownKey(_) => "unknown key",
            ConfigError::InvalidValueType(_, _, _) => "invalid value type"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ConfigError::File(ref err) => Some(err),
            ConfigError::ParseFloat(ref err) => Some(err),
            ConfigError::ParseInt(ref err) => Some(err),
            ConfigError::InvalidFieldCount(_, _, _) => None,
            ConfigError::InvalidFieldType(_, _) => None,
            ConfigError::UnknownKey(_) => None,
            ConfigError::InvalidValueType(_, _, _) => None
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigError::File(ref err) => write!(f, "{}: {}", self.description(), err),
            ConfigError::ParseFloat(ref err) => write!(f, "{}: {}", self.description(), err),
            ConfigError::ParseInt(ref err) => write!(f, "{}: {}", self.description(), err),
            ConfigError::InvalidFieldCount(expect_count, got_count, ref line_str) => write!(f, "{}: expected {} fields, but got {}, line: '{}'",
                self.description(), expect_count, got_count, line_str),
            ConfigError::InvalidFieldType(ref unknown_field_type, ref line_str) => write!(f, "{}: '{}', line: '{}'",
                self.description(), unknown_field_type, line_str),
            ConfigError::UnknownKey(ref unknown_key) => write!(f, "{}: '{}'", self.description(), unknown_key),
            ConfigError::InvalidValueType(ref key, ref expected_type, ref is_type) => write!(f, "{}: key = '{}', requested type = '{}', is type = '{}' ",
                self.description(), key, expected_type, is_type)
        }
    }
}
