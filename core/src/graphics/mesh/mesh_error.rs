use thiserror::Error;

use crate::file::FileError;
use crate::graphics::OpenglError;

#[derive(Error, Debug)]
pub enum MeshError {
    #[error("file: {source}")]
    File {
        #[from]
        source: FileError,
    },
    #[error("opengl: {source}")]
    Opengl {
        #[from]
        source: OpenglError,
    },
    #[error("mesh with id '{0}' not existing")]
    MeshNotFound(String),
}
