use gl;
use gl::types::{GLenum, GLint, GLuint};
use std::ptr;

use super::utility::{delete_vao, delete_vbos};
use crate::graphics::{check_opengl_error, GraphicsError};
use crate::traits::{RenderInfo, Renderable};

pub struct VAO {
    vao: GLuint,
    vbos: Vec<GLuint>,
    element_type: GLenum,
    index_count: GLint,
}

impl VAO {
    pub fn new(vao: GLuint, vbos: &[GLuint], element_type: GLenum, index_count: GLint) -> Self {
        VAO {
            vao: vao,
            vbos: vbos.into(),
            element_type: element_type,
            index_count: index_count,
        }
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count as u32
    }
}

impl Renderable for VAO {
    fn render<'a>(&self, _info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                self.element_type,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl::BindVertexArray(0);
        }
        match check_opengl_error("VAO::render") {
            Ok(_) => Ok(()),
            Err(e) => Err(GraphicsError::from(e)),
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        if let Err(e) = delete_vbos(&self.vbos) {
            error!("{}", e);
        }
        if let Err(e) = delete_vao(self.vao) {
            error!("{}", e);
        }
    }
}
