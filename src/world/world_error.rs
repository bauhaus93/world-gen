use std::fmt;
use std::error::Error;
use std::io;

use crate::graphics::{ GraphicsError, mesh::MeshError };

#[derive(Debug)]
pub enum WorldError {
    Graphics(GraphicsError),
    MeshCreation(MeshError)
}

impl From<GraphicsError> for WorldError {
    fn from(err: GraphicsError) -> Self {
        WorldError::Graphics(err)
    }
}

impl From<MeshError> for WorldError {
    fn from(err: MeshError) -> Self {
        WorldError::MeshCreation(err)
    }
}

impl Error for WorldError {

    fn description(&self) -> &str {
        match *self {
            WorldError::Graphics(_) => "graphics",
            WorldError::MeshCreation(_) => "mesh creation",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WorldError::Graphics(ref err) => Some(err),
            WorldError::MeshCreation(ref err) => Some(err),
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::MeshCreation(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}


