use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum UpdateError {
	Resource(String)
}

impl Error for UpdateError {

    fn description(&self) -> &str {
        match *self {
            UpdateError::Resource(_) => "resource"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            UpdateError::Resource(_) => None, 
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateError::Resource(ref s) => write!(f, "{}:{}", self.description(), s)
        }
    }
}
