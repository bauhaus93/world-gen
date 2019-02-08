use std::fmt;
use std::error::Error;
use std::io;

use crate::graphics::{ GraphicsError, mesh::MeshError };
use super::chunk::ChunkError;

#[derive(Debug)]
pub enum WorldError {
    Graphics(GraphicsError),
    Mesh(MeshError),
    Chunk(ChunkError)
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

impl Error for WorldError {

    fn description(&self) -> &str {
        match *self {
            WorldError::Graphics(_) => "graphics",
            WorldError::Mesh(_) => "mesh",
            WorldError::Chunk(_) => "chunk",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            WorldError::Graphics(ref err) => Some(err),
            WorldError::Mesh(ref err) => Some(err),
            WorldError::Chunk(ref err) => Some(err),
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Chunk(ref err) => write!(f, "{}/{}", self.description(), err),
        }
    }
}


