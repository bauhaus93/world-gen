use std::fmt;
use std::error::Error;

use serde_yaml;

use core::{ CoreError, config::ConfigError, file::FileError, object::ObjectError };
use core::graphics::{ GraphicsError, mesh::MeshError };
use super::chunk::ChunkError;

#[derive(Debug)]
pub enum WorldError {
    Graphics(GraphicsError),
	Core(CoreError),
    Mesh(MeshError),
    Chunk(ChunkError),
    Object(ObjectError),
    Config(ConfigError),
    File(FileError),
    Yaml(serde_yaml::Error)
}

impl From<CoreError> for WorldError {
    fn from(err: CoreError) -> Self {
        WorldError::Core(err)
    }
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

impl From<FileError> for WorldError {
    fn from(err: FileError) -> Self {
        WorldError::File(err)
    }
}

impl From<serde_yaml::Error> for WorldError {
    fn from(err: serde_yaml::Error) -> Self {
        WorldError::Yaml(err)
    }
}

impl Error for WorldError {

    fn description(&self) -> &str {
        match *self {
			WorldError::Core(_) => "core",
            WorldError::Graphics(_) => "graphics",
            WorldError::Mesh(_) => "mesh",
            WorldError::Chunk(_) => "chunk",
            WorldError::Object(_) => "object",
            WorldError::Config(_) => "config",
            WorldError::File(_) => "file",
            WorldError::Yaml(_) => "yaml"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            WorldError::Core(ref err) => Some(err),
            WorldError::Graphics(ref err) => Some(err),
            WorldError::Mesh(ref err) => Some(err),
            WorldError::Chunk(ref err) => Some(err),
            WorldError::Object(ref err) => Some(err),
            WorldError::Config(ref err) => Some(err),
            WorldError::File(ref err) => Some(err),
            WorldError::Yaml(ref err) => Some(err)
        }
    }
}

impl fmt::Display for WorldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorldError::Core(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Graphics(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Chunk(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Object(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Config(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            WorldError::Yaml(ref err) => write!(f, "{}/{}", self.description(), err)
        }
    }
}


