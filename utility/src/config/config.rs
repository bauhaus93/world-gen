use std::collections::{ BTreeMap };

use crate::read_file;
use super::{ Value, ConfigError };


pub struct Config {
    entry_map: BTreeMap<String, Value> 
}

impl Config {
    pub fn read(config_path: &str) -> Result<Config, ConfigError> {
        let content = read_file(config_path)?;
        let mut entry_map = BTreeMap::new();

        for line in content.lines() {
            if line.len() > 0 {
                let (key, value) = parse_line(line)?;
                if let Some(_old) = entry_map.insert(key.clone(), value) {
                    warn!("Duplicate key '{}' in file, overwriting older entry.", key);
                }
            }
        }

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

fn parse_line(line: &str) -> Result<(String, Value), ConfigError> {
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() != 3 {
        return Err(ConfigError::InvalidFieldCount(3, fields.len(), line.to_owned()));
    }
    let key = fields[0].to_lowercase().to_owned();
    let value = match (fields[1].to_lowercase().as_str(), fields[2]) {
        ("str", raw_value) => Value::Str(raw_value.to_owned()),
        ("int", raw_value) => Value::Int(raw_value.parse()?),
        ("uint", raw_value) => Value::Uint(raw_value.parse()?),
        ("float", raw_value) => Value::Float(raw_value.parse()?),
        (unknown_type, _) => {
            return Err(ConfigError::InvalidFieldType(unknown_type.to_owned(), line.to_owned()));
        }
    };

    Ok((key, value))
}
