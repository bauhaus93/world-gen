use std::convert::TryInto;

use gl;
use gl::types::GLuint;

use crate::graphics::{ check_opengl_error, OpenglError };

pub fn delete_vao(vao: GLuint) -> Result<(), OpenglError> {
    unsafe { gl::DeleteVertexArrays(1, &vao as * const GLuint); }
    check_opengl_error("gl::DeleteVertexArrays")
}

pub fn delete_vbos(vbos: &[GLuint]) -> Result<(), OpenglError> {
    unsafe {
        gl::DeleteBuffers(
            vbos.len().try_into().unwrap(),
            vbos.as_ptr() as * const GLuint
        );
        check_opengl_error("gl::DeleteBuffers")?;
    }
    Ok(())
}
