use thiserror::Error;

use core::config::ConfigError;
use core::graphics::{mesh::MeshError, GraphicsError};
use core::object::ObjectError;

#[derive(Error, Debug)]
pub enum ChunkError {
    #[error("mesh: {source}")]
    Mesh {
        #[from]
        source: MeshError,
    },
    #[error("object: {source}")]
    Object {
        #[from]
        source: ObjectError,
    },
    #[error("graphics: {source}")]
    Graphics {
        #[from]
        source: GraphicsError,
    },
    #[error("config: {source}")]
    Config {
        #[from]
        source: ConfigError,
    },

    #[error("no buffer built: chunk pos = {0}/{1}")]
    NoBufferBuilt(i32, i32),
    #[error("coult not triangulate heightmap")]
    HeightmapTriangulation,
    #[error("mutex poisoned")]
    MutexPoison,
}
