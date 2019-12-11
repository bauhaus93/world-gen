use std::collections::{ BTreeMap };

use serde_yaml;

use crate::file::read_file;
use super::{ Value, ConfigError };


pub struct Config {
    entry_map: BTreeMap<String, Value> 
}

impl Config {
    pub fn read(config_path: &str) -> Result<Config, ConfigError> {
        let content = read_file(config_path)?;
        let entry_map: BTreeMap<String, Value> = serde_yaml::from_str(&content)?;

        debug!("Read {} key/value pairs from config file '{}'", entry_map.len(), config_path);

        let config = Config {
            entry_map: entry_map
        };
        Ok(config)
    }

    pub fn get_str(&self, key: &str) -> Result<&str, ConfigError> {
        match self.entry_map.get(key) {
            Some(Value::Str(str_value)) => Ok(str_value),
            Some(other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), Value::Str("".to_owned()), other_val.clone())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_str_or_default(&self, key: &str, default: &str) -> String {
        match self.entry_map.get(key) {
            Some(Value::Str(str_value)) => str_value.to_owned(),
            _ => default.to_owned()
        } 
    }

    pub fn get_int(&self, key: &str) -> Result<i32, ConfigError> {
        match self.entry_map.get(key) {
            Some(Value::Int(int_value)) => Ok(*int_value),
            Some(other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), Value::Int(0), other_val.clone())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_int_or_default(&self, key: &str, default: i32) -> i32 {
        match self.entry_map.get(key) {
            Some(Value::Int(int_value)) => *int_value,
            _ => default
        }
    }

    pub fn get_uint(&self, key: &str) -> Result<u32, ConfigError> {
        match self.entry_map.get(key) {
            Some(Value::Uint(uint_value)) => Ok(*uint_value),
            Some(other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), Value::Uint(0), other_val.clone())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_uint_or_default(&self, key: &str, default: u32) -> u32 {
        match self.entry_map.get(key) {
            Some(Value::Uint(uint_value)) => *uint_value,
            _ => default
        }
    }

    pub fn get_float(&self, key: &str) -> Result<f32, ConfigError> {
        match self.entry_map.get(key) {
            Some(Value::Float(float_value)) => Ok(*float_value),
            Some(other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), Value::Float(0.), other_val.clone())),
            None => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_float_or_default(&self, key: &str, default: f32) -> f32 {
        match self.entry_map.get(key) {
            Some(Value::Float(float_value)) => *float_value,
            _ => default
        }
    }
}
