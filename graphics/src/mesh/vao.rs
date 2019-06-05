use std::ptr;
use gl;
use gl::types::{ GLint, GLuint, GLenum };

use crate::{ check_opengl_error };
use crate::mesh::MeshError;
use crate::mesh::utility::{ delete_vao, delete_vbos };

pub struct VAO {
    vao: GLuint,
    vbos: Vec<GLuint>,
    element_type: GLenum,
    index_count: GLuint
}

impl VAO {

    pub fn new(vao: GLuint, vbos: &[GLuint], element_type: GLenum, index_count: GLuint) -> Self {
        VAO {
            vao: vao,
            vbos: vbos.into(),
            element_type: element_type,
            index_count: index_count
        }
    }

    pub fn get_index_count(&self) -> u32 {
        self.index_count as u32
    }

    pub fn render(&self) -> Result<(), MeshError> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                self.element_type,
                self.index_count as GLint,
                gl::UNSIGNED_INT,
                ptr::null()
            );
            gl::BindVertexArray(0);
        }
        check_opengl_error("Mesh::render")?;
        Ok(())
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        if let Err(e) =  delete_vbos(&self.vbos) {
            error!("{}", e);
        }
        if let Err(e) =  delete_vao(self.vao){
            error!("{}", e);
        }
    }
}