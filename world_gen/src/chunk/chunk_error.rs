use std::fmt;
use std::error::Error;

use graphics::mesh::MeshError;

#[derive(Debug)]
pub enum ChunkError {
    Mesh(MeshError),
    NoBufferBuilt([i32; 2]),
    MutexPoison
}

impl From<MeshError> for ChunkError {
    fn from(err: MeshError) -> Self {
        ChunkError::Mesh(err)
    }
}

impl Error for ChunkError {

    fn description(&self) -> &str {
        match *self {
            ChunkError::Mesh(_) => "mesh",
            ChunkError::NoBufferBuilt(_) => "no buffer built",
            ChunkError::MutexPoison => "mutex poison"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ChunkError::Mesh(ref err) => Some(err),
            ChunkError::NoBufferBuilt(_) => None,
            ChunkError::MutexPoison => None
        }
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChunkError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            ChunkError::NoBufferBuilt(chunk_pos) => write!(f, "{}: chunk pos = {}/{}", self.description(), chunk_pos[0], chunk_pos[1]),
            ChunkError::MutexPoison => write!(f, "{}", self.description())
        }
    }
}


