use thiserror::Error;

use gl::types::GLuint;

use crate::file::FileError;
use crate::graphics::OpenglError;

#[derive(Error, Debug)]
pub enum ShaderError {
    #[error("file: {source}")]
    File {
        #[from]
        source: FileError,
    },
    #[error("unknown shader type: {0}")]
    UnknownShaderType(GLuint),
    #[error("compilation: {0}")]
    Compilation(String),
    #[error("opengl: {source}")]
    Opengl {
        #[from]
        source: OpenglError,
    },
    #[error("function failure: {0}")]
    FunctionFailure(String),
}
