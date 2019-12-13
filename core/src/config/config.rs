use std::collections::{ BTreeMap };

use serde_yaml;

use crate::file::read_file;
use super::ConfigError;

pub struct Config {
    entry_map: BTreeMap<String, serde_yaml::Value> 
}

impl Config {
    pub fn read(config_path: &str) -> Result<Config, ConfigError> {
        let content = read_file(config_path)?;
        let entry_map: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(&content)?;

        debug!("Read {} key/value pairs from config file '{}'", entry_map.len(), config_path);

        let config = Config {
            entry_map: entry_map
        };
        Ok(config)
    }

    pub fn get_str(&self, key: &str) -> Result<&str, ConfigError> {
        match self.entry_map.get(key) {
            Some(v) if v.is_string() => Ok(v.as_str().unwrap()),
            Some(_other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), "string".to_owned())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_str_or_default(&self, key: &str, default: &str) -> String {
        match self.get_str(key) {
            Ok(v) => v.to_owned(),
            Err(_) => default.to_owned()
        }
    }

    pub fn get_int(&self, key: &str) -> Result<i32, ConfigError> {
        match self.entry_map.get(key) {
            Some(v) if v.is_i64() => Ok(v.as_i64().unwrap() as i32),
            Some(_other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), "integer".to_owned())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_int_or_default(&self, key: &str, default: i32) -> i32 {
        match self.get_int(key) {
            Ok(v) => v,
            Err(_) => default
        }
    }

    pub fn get_uint(&self, key: &str) -> Result<u32, ConfigError> {
        match self.entry_map.get(key) {
            Some(v) if v.is_u64() => Ok(v.as_u64().unwrap() as u32),
            Some(_other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), "unsigned integer".to_owned())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_uint_or_default(&self, key: &str, default: u32) -> u32 {
        match self.get_uint(key) {
            Ok(v) => v,
            Err(_) => default
        }
    }

    pub fn get_float(&self, key: &str) -> Result<f32, ConfigError> {
        match self.entry_map.get(key) {
            Some(v) if v.is_f64() => Ok(v.as_f64().unwrap() as f32),
            Some(_other_val) => Err(ConfigError::InvalidValueType(key.to_owned(), "float".to_owned())),
            _ => Err(ConfigError::UnknownKey(key.to_owned()))
        }
    }

    pub fn get_float_or_default(&self, key: &str, default: f32) -> f32 {
        match self.get_float(key) {
            Ok(v) => v,
            Err(_) => default
        }
    }
}
