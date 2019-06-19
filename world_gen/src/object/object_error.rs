use std::fmt;
use std::error::Error;

use serde_json;

use graphics::mesh::MeshError;
use utility::FileError;

#[derive(Debug)]
pub enum ObjectError {
    Mesh(MeshError),
    File(FileError),
    Json(serde_json::Error),
    PrototypeNotExisting(String)
}

impl From<MeshError> for ObjectError {
    fn from(err: MeshError) -> Self {
        ObjectError::Mesh(err)
    }
}

impl From<FileError> for ObjectError {
    fn from(err: FileError) -> Self {
        ObjectError::File(err)
    }
}

impl From<serde_json::Error> for ObjectError {
    fn from(err: serde_json::Error) -> Self {
        ObjectError::Json(err)
    }
}


impl Error for ObjectError {

    fn description(&self) -> &str {
        match *self {
            ObjectError::Mesh(_) => "mesh",
            ObjectError::File(_) => "file",
            ObjectError::Json(_) => "json",
            ObjectError::PrototypeNotExisting(_) => "protoype not existing",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ObjectError::Mesh(ref err) => Some(err),
            ObjectError::File(ref err) => Some(err),
            ObjectError::Json(ref err) => Some(err),
            ObjectError::PrototypeNotExisting(_) => None,
        }
    }
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            ObjectError::File(ref err) => write!(f, "{}/{}", self.description(), err),
            ObjectError::Json(ref err) => write!(f, "{}/{}", self.description(), err),
            ObjectError::PrototypeNotExisting(ref name) => write!(f, "{}: '{}'", self.description(), name)
        }
    }
}


