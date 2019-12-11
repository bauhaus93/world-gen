use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum UpdateError {
	SomeError
}

/*impl From<utility::ConfigError> for CoreError {
    fn from(err: utility::ConfigError) -> Self {
        CoreError::Config(err)
    }
}*/

impl Error for UpdateError {

    fn description(&self) -> &str {
        match *self {
            UpdateError::SomeError => "Placeholder error"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            UpdateError::SomeError => None
        }
    }
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateError::SomeError => write!(f, "{}: Generic placeholder", self.description())
        }
    }
}
