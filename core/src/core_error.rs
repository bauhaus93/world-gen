use std::fmt;
use std::error::Error;

use crate::{graphics::GraphicsError, file::FileError, config::ConfigError };

#[derive(Debug)]
pub enum CoreError {
    Graphics(GraphicsError),
    File(FileError),
    Config(ConfigError),
    InvalidWindowSize(f64, f64)
}

impl From<GraphicsError> for CoreError {
    fn from(err: GraphicsError) -> Self {
        CoreError::Graphics(err)
    }
}

impl From<FileError> for CoreError {
    fn from(err: FileError) -> Self {
        CoreError::File(err)
    }
}

impl From<ConfigError> for CoreError {
    fn from(err: ConfigError) -> Self {
        CoreError::Config(err)
    }
}

impl Error for CoreError {

    fn description(&self) -> &str {
        match *self {
            CoreError::Graphics(_) => "graphics",
            CoreError::File(_) => "file",
            CoreError::Config(_) => "config",
            CoreError::InvalidWindowSize(_, _) => "invalid window size"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            CoreError::Graphics(ref err) => Some(err),
            CoreError::File(ref err) => Some(err),
            CoreError::Config(ref err) => Some(err),
            CoreError::InvalidWindowSize(_, _) => None
        }
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CoreError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            CoreError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            CoreError::Config(ref err) => write!(f, "{}/{}", self.description(), err),
            CoreError::InvalidWindowSize(x, y) => write!(f, "{}: {}x{}, values must be > 0", self.description(), x, y)
        }
    }
}
