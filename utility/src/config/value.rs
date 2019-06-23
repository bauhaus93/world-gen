use std::fmt;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Value {
    Str(String),
    Int(i32),
    Uint(u32),
    Float(f32)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(v) => write!(f, "String('{}')", v),
            Value::Int(v) => write!(f, "Integer({})", v),
            Value::Uint(v) => write!(f, "UnsignedInteger({})", v),
            Value::Float(v) => write!(f, "Float({})", v)
        }
    }
}