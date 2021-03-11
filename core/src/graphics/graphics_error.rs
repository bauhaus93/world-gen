use thiserror::Error;

use glutin;
use image;

use crate::graphics::{
    mesh::MeshError,
    shader::{ShaderError, ShaderProgramError},
    OpenglError,
};

#[derive(Error, Debug)]
pub enum GraphicsError {
    #[error("glutin creation: {source}")]
    GlutinCreation {
        #[from]
        source: glutin::CreationError,
    },
    #[error("glutin context: {source}")]
    GlutinContext {
        #[from]
        source: glutin::ContextError,
    },
    #[error("image: {source}")]
    Image {
        #[from]
        source: image::ImageError,
    },
    #[error("shader: {source}")]
    Shader {
        #[from]
        source: ShaderError,
    },
    #[error("mesh: {source}")]
    Mesh {
        #[from]
        source: MeshError,
    },
    #[error("shader program: {source}")]
    ShaderProgram {
        #[from]
        source: ShaderProgramError,
    },
    #[error("opengl: {source}")]
    Opengl {
        #[from]
        source: OpenglError,
    },
    #[error("function failure: {0}")]
    FunctionFailure(String),
    #[error("invalid image format: {0}")]
    InvalidImageFormat(String),
}
