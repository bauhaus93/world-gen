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
            let (key, value) = parse_line(line)?;
            if let Some(_old) = entry_map.insert(key.clone(), value) {
                warn!("Duplicate key '{}' in file, overwriting older entry.", key);
            }
        }

        debug!("Read {} key/value pairs from config file '{}'", entry_map.len(), config_path);


        let config = Config {
            entry_map: entry_map
        };
        Ok(config)
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        match self.entry_map.get(key) {
            Some(Value::Str(str_value)) => Some(str_value),
            _ => None
        }
    }

    pub fn get_str_or_default(&self, key: &str, default: &str) -> String {
        match self.entry_map.get(key) {
            Some(Value::Str(str_value)) => str_value.to_owned(),
            _ => default.to_owned()
        } 
    }

    pub fn get_int(&self, key: &str) -> Option<i32> {
        match self.entry_map.get(key) {
            Some(Value::Int(int_value)) => Some(*int_value),
            _ => None
        }
    }

    pub fn get_int_or_default(&self, key: &str, default: i32) -> i32 {
        match self.entry_map.get(key) {
            Some(Value::Int(int_value)) => *int_value,
            _ => default
        }
    }

    pub fn get_float(&self, key: &str) -> Option<f32> {
        match self.entry_map.get(key) {
            Some(Value::Float(float_value)) => Some(*float_value),
            _ => None
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
        ("float", raw_value) => Value::Float(raw_value.parse()?),
        (unknown_type, _) => {
            return Err(ConfigError::InvalidFieldType(unknown_type.to_owned(), line.to_owned()));
        }
    };

    Ok((key, value))
}
