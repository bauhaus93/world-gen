use std::fmt;
use std::error::Error;

use gl::types::{ GLuint };

use crate::file::FileError;
use crate::graphics::OpenglError;

#[derive(Debug)]
pub enum ShaderError {
    File(FileError),
    UnknownShaderType(GLuint),
    Compilation(String),
    Opengl(OpenglError),
    FunctionFailure(String)
}

impl From<FileError> for ShaderError {
    fn from(err: FileError) -> Self {
        ShaderError::File(err)
    }
}

impl From<OpenglError> for ShaderError {
    fn from(err: OpenglError) -> Self {
        ShaderError::Opengl(err)
    }
}

impl Error for ShaderError {

    fn description(&self) -> &str {
        match *self {
            ShaderError::File(_) => "file",
            ShaderError::UnknownShaderType(_) => "unknown shader type",
            ShaderError::Compilation(_) => "compilation",
            ShaderError::Opengl(_) => "opengl",
            ShaderError::FunctionFailure(_) => "function failure"
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ShaderError::File(ref err) => Some(err),
            ShaderError::UnknownShaderType(_) => None,
            ShaderError::Compilation(_) => None,
            ShaderError::Opengl(ref err) => Some(err),
            ShaderError::FunctionFailure(_) => None
        }
    }
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            ShaderError::UnknownShaderType(type_id) => write!(f, "{}: type id is {}", self.description(), type_id),
            ShaderError::Compilation(ref shader_log) => write!(f, "{}: {}", self.description(), shader_log),
            ShaderError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            ShaderError::FunctionFailure(ref func_name) => write!(f, "{} @ {}", self.description(), func_name)
        }
    }
}


