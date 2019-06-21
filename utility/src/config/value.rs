use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    Int(i32),
    Uint(u32),
    Float(f32)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Str(_) => write!(f, "String"),
            Value::Int(_) => write!(f, "Integer"),
            Value::Uint(_) => write!(f, "UnsignedInteger"),
            Value::Float(_) => write!(f, "Float")
        }
    }
}