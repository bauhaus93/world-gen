use thiserror::Error;

use gl;
use gl::types::GLuint;

#[derive(Error, Debug)]
pub enum OpenglError {
    #[error("invalid value: {0}")]
    InvalidValue(String),
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
    #[error("invalid enum: {0}")]
    InvalidEnum(String),
    #[error("stack overflow: {0}")]
    StackOverflow(String),
    #[error("stack underflow: {0}")]
    StackUnderflow(String),
    #[error("out of memory: {0}")]
    OutOfMemory(String),
    #[error("invalid framebuffer operation: {0}")]
    InvalidFramebufferOperation(String),
    #[error("context lost: {0}")]
    ContextLost(String),
}

pub fn check_opengl_error(func_name: &str) -> Result<(), OpenglError> {
    let func_string = func_name.to_string();
    match get_opengl_error_code() {
        gl::NO_ERROR => Ok(()),
        gl::INVALID_VALUE => Err(OpenglError::InvalidValue(func_string)),
        gl::INVALID_OPERATION => Err(OpenglError::InvalidOperation(func_string)),
        gl::INVALID_ENUM => Err(OpenglError::InvalidEnum(func_string)),
        gl::STACK_OVERFLOW => Err(OpenglError::StackOverflow(func_string)),
        gl::STACK_UNDERFLOW => Err(OpenglError::StackUnderflow(func_string)),
        gl::OUT_OF_MEMORY => Err(OpenglError::OutOfMemory(func_string)),
        gl::INVALID_FRAMEBUFFER_OPERATION => {
            Err(OpenglError::InvalidFramebufferOperation(func_string))
        }
        gl::CONTEXT_LOST => Err(OpenglError::ContextLost(func_string)),
        _ => unreachable!(),
    }
}

fn get_opengl_error_code() -> GLuint {
    unsafe { gl::GetError() }
}
