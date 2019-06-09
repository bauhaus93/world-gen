use std::fmt;
use std::error::Error;

use graphics::mesh::MeshError;

#[derive(Debug)]
pub enum ObjectError {
    Mesh(MeshError),
    PrototypeNotExisting(String)
}

impl From<MeshError> for ObjectError {
    fn from(err: MeshError) -> Self {
        ObjectError::Mesh(err)
    }
}

impl Error for ObjectError {

    fn description(&self) -> &str {
        match *self {
            ObjectError::Mesh(_) => "mesh",
            ObjectError::PrototypeNotExisting(_) => "protoype not existing",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            ObjectError::Mesh(ref err) => Some(err),
            ObjectError::PrototypeNotExisting(_) => None,
        }
    }
}

impl fmt::Display for ObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ObjectError::Mesh(ref err) => write!(f, "{}/{}", self.description(), err),
            ObjectError::PrototypeNotExisting(ref name) => write!(f, "{}: '{}'", self.description(), name)
        }
    }
}


