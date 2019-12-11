use std::fmt;
use std::error::Error;

use core::{UpdateError, CoreError, config::ConfigError};
use world::WorldError;

#[derive(Debug)]
pub enum ApplicationError {
    Core(CoreError),
	Update(UpdateError),
	Config(ConfigError),
	World(WorldError)
}

impl From<CoreError> for ApplicationError {
    fn from(err: CoreError) -> Self {
        ApplicationError::Core(err)
    }
}

impl From<UpdateError> for ApplicationError {
    fn from(err: UpdateError) -> Self {
        ApplicationError::Update(err)
    }
}

impl From<ConfigError> for ApplicationError {
    fn from(err: ConfigError) -> Self {
        ApplicationError::Config(err)
    }
}

impl From<WorldError> for ApplicationError {
    fn from(err: WorldError) -> Self {
        ApplicationError::World(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Core(_) => "core",
            ApplicationError::Update(_) => "update",
            ApplicationError::Config(_) => "config",
            ApplicationError::World(_) => "world",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ApplicationError::Core(ref err) => Some(err),
            ApplicationError::Update(ref err) => Some(err),
            ApplicationError::Config(ref err) => Some(err),
            ApplicationError::World(ref err) => Some(err),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Core(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Update(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Config(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::World(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}
