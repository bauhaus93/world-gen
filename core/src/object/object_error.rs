use serde_yaml;
use thiserror::Error;

use crate::file::FileError;
use crate::graphics::mesh::MeshError;

#[derive(Error, Debug)]
pub enum ObjectError {
    #[error("mesh: {source}")]
    Mesh {
        #[from]
        source: MeshError,
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
    #[error("prototype not existing: {0}")]
    PrototypeNotExisting(String),
}
