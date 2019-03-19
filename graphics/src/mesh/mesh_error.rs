use std::fmt;
use std::error::Error;

use utility::FileError;
use crate::OpenglError;

#[derive(Debug)]
pub enum MeshError {
    File(FileError),
    Opengl(OpenglError),
    MeshNotFound(String)
}

impl From<FileError> for MeshError {
    fn from(err: FileError) -> Self {
        MeshError::File(err)
    }
}

impl From<OpenglError> for MeshError {
    fn from(err: OpenglError) -> Self {
        MeshError::Opengl(err)
    }
}

impl Error for MeshError {

    fn description(&self) -> &str {
        match *self {
            MeshError::File(_) => "file",
            MeshError::Opengl(_) => "opengl",
            MeshError::MeshNotFound(_) => "mesh not found"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            MeshError::File(ref err) => Some(err),
            MeshError::Opengl(ref err) => Some(err),
            MeshError::MeshNotFound(_) => None
        }
    }
}

impl fmt::Display for MeshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MeshError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            MeshError::Opengl(ref err) => write!(f, "{}/{}", self.description(), err),
            MeshError::MeshNotFound(ref s) => write!(f, "{}: id '{}' not existing", self.description(), s)
        }
    }
}


