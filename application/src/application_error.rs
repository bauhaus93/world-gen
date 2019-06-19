use std::fmt;
use std::error::Error;

use utility;
use graphics;
use world_gen;

#[derive(Debug)]
pub enum ApplicationError {
    Graphics(graphics::GraphicsError),
    World(world_gen::WorldError),
    File(utility::FileError),
    Config(utility::ConfigError)
}

impl From<graphics::GraphicsError> for ApplicationError {
    fn from(err: graphics::GraphicsError) -> Self {
        ApplicationError::Graphics(err)
    }
}

impl From<world_gen::WorldError> for ApplicationError {
    fn from(err: world_gen::WorldError) -> Self {
        ApplicationError::World(err)
    }
}

impl From<utility::FileError> for ApplicationError {
    fn from(err: utility::FileError) -> Self {
        ApplicationError::File(err)
    }
}

impl From<utility::ConfigError> for ApplicationError {
    fn from(err: utility::ConfigError) -> Self {
        ApplicationError::Config(err)
    }
}

impl Error for ApplicationError {

    fn description(&self) -> &str {
        match *self {
            ApplicationError::Graphics(_) => "graphics",
            ApplicationError::World(_) => "world",
            ApplicationError::File(_) => "file",
            ApplicationError::Config(_) => "config"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ApplicationError::Graphics(ref err) => Some(err),
            ApplicationError::World(ref err) => Some(err),
            ApplicationError::File(ref err) => Some(err),
            ApplicationError::Config(ref err) => Some(err)
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApplicationError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::World(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            ApplicationError::Config(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}
