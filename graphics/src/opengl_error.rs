use std::fmt;
use std::error::Error;

use gl;
use gl::types::GLuint;

#[derive(Debug)]
pub enum OpenglError {
    InvalidValue(String),
    InvalidOperation(String),
    InvalidEnum(String),
    StackOverflow(String),
    StackUnderflow(String),
    OutOfMemory(String),
    InvalidFramebufferOperation(String),
    ContextLost(String)
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
        gl::INVALID_FRAMEBUFFER_OPERATION => Err(OpenglError::InvalidFramebufferOperation(func_string)),
        gl::CONTEXT_LOST => Err(OpenglError::ContextLost(func_string)),
        _ => unreachable!()
    }
}

impl Error for OpenglError {
    fn description(&self) -> &str {
        match *self {
            OpenglError::InvalidValue(_) => "invalid value",
            OpenglError::InvalidOperation(_) => "invalid operation",
            OpenglError::InvalidEnum(_) => "invalid enum",
            OpenglError::StackOverflow(_) => "stack overflow",
            OpenglError::StackUnderflow(_) => "stack underflow",
            OpenglError::OutOfMemory(_) => "out of memory",
            OpenglError::InvalidFramebufferOperation(_) => "invalid framebuffer operation",
            OpenglError::ContextLost(_) => "context lost"
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for OpenglError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpenglError::InvalidValue(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::InvalidOperation(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::InvalidEnum(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::StackOverflow(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::StackUnderflow(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::OutOfMemory(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::InvalidFramebufferOperation(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
            OpenglError::ContextLost(ref func_name) => write!(f, "{} @ {}", self.description(), func_name),
        }
    }
}

fn get_opengl_error_code() -> GLuint {
    unsafe {
        gl::GetError()    
    }
}
