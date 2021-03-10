use std::error::Error;
use std::fmt;

use crate::{config::ConfigError, graphics::GraphicsError, UpdateError};

#[derive(Debug)]
pub enum StateError {
    Update(UpdateError),
    Config(ConfigError),
    Graphics(GraphicsError),
}

impl From<UpdateError> for StateError {
    fn from(err: UpdateError) -> Self {
        StateError::Update(err)
    }
}

impl From<ConfigError> for StateError {
    fn from(err: ConfigError) -> Self {
        StateError::Config(err)
    }
}

impl From<GraphicsError> for StateError {
    fn from(err: GraphicsError) -> Self {
        StateError::Graphics(err)
    }
}

impl Error for StateError {
    fn description(&self) -> &str {
        match *self {
            StateError::Update(_) => "update",
            StateError::Config(_) => "config",
            StateError::Graphics(_) => "graphics",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            StateError::Update(ref err) => Some(err),
            StateError::Config(ref err) => Some(err),
            StateError::Graphics(ref err) => Some(err),
        }
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StateError::Update(ref err) => write!(f, "{}/{}", self.description(), err),
            StateError::Config(ref err) => write!(f, "{}/{}", self.description(), err),
            StateError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}
