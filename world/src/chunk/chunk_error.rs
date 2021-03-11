use thiserror::Error;

use core::graphics::mesh::MeshError;
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
    #[error("no buffer built: chunk pos = {0}/{1}")]
    NoBufferBuilt(i32, i32),
    #[error("mutex poisoned")]
    MutexPoison,
}
