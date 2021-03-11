use thiserror::Error;

use crate::graphics::OpenglError;

#[derive(Error, Debug)]
pub enum ShaderProgramError {
    #[error("linker: {0}")]
    Linkage(String),
    #[error("opengl: {source}")]
    Opengl {
        #[from]
        source: OpenglError,
    },
    #[error("function failure: {0}")]
    FunctionFailure(String),
    #[error("handle not existing: {0}")]
    HandleNotExisting(String),
}
