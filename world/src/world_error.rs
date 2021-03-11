use serde_yaml;
use thiserror::Error;

use crate::chunk::ChunkError;
use core::graphics::{mesh::MeshError, GraphicsError};
use core::{config::ConfigError, file::FileError, object::ObjectError, CoreError};

#[derive(Error, Debug)]
pub enum WorldError {
    #[error("graphics: {source}")]
    Graphics {
        #[from]
        source: GraphicsError,
    },
    #[error("core: {source}")]
    Core {
        #[from]
        source: CoreError,
    },
    #[error("mesh: {source}")]
    Mesh {
        #[from]
        source: MeshError,
    },
    #[error("chunk: {source}")]
    Chunk {
        #[from]
        source: ChunkError,
    },
    #[error("object: {source}")]
    Object {
        #[from]
        source: ObjectError,
    },
    #[error("config: {source}")]
    Config {
        #[from]
        source: ConfigError,
    },
    #[error("file: {source}")]
    File {
        #[from]
        source: FileError,
    },
    #[error("yaml: {source}")]
    Yaml {
        #[from]
        source: serde_yaml::Error,
    },
}
