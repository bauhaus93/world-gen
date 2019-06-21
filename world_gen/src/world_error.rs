use std::fmt;
use std::error::Error;

use utility::ConfigError;
use graphics::{ GraphicsError, mesh::MeshError };
use super::chunk::chunk_error::ChunkError;
use super::object::object_error::ObjectError;

#[derive(Debug)]
pub enum WorldError {
    Graphics(GraphicsError),
    Mesh(MeshError),
    Chunk(ChunkError),
    Object(ObjectError),
    Config(ConfigError)
}

impl From<GraphicsError> for WorldError {
    fn from(err: GraphicsError) -> Self {
        WorldError::Graphics(err)
    }
}

impl From<MeshError> for WorldError {
    fn from(err: MeshError) -> Self {
        WorldError::Mesh(err)
    }
}

impl From<ChunkError> for WorldError {
    fn from(err: ChunkError) -> Self {
        WorldError::Chunk(err)
    }
}

impl From<ObjectError> for WorldError {
    fn from(err: ObjectError) -> Self {
        WorldError::Object(err)
    }
}

impl From<ConfigError> for WorldError {
    fn from(err: ConfigError) -> Self {
        WorldError::Config(err)
    }
}

impl Error for WorldError {

    fn description(&self) -> &str {
        match *self {
            WorldError::Graphics(_) => "graphics",
            WorldError::Mesh(_) => "mesh",
            WorldError::Chunk(_) => "chunk",
            WorldError::Object(_) => "object",
            WorldError::Config(_) => "config"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            WorldError::Graphics(ref err) => Some(err),
            WorldError::Mesh(ref err) => Some(err),
            WorldError::Chunk(ref err) => Some(err),
            WorldError::Object(ref err) => Some(err),
            WorldError::Config(ref err) => Some(err)
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Chunk(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Object(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Config(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}


