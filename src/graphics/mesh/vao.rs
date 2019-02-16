use std::ptr;
use std::convert::TryFrom;
use gl;
use gl::types::{ GLint, GLuint, GLenum };

use crate::graphics::{ check_opengl_error, OpenglError };
use super::{ MeshError, Buffer };

pub struct VAO {
    vao: GLuint,
    vbos: [GLuint; 4],
    index_count: GLuint
}

impl VAO {
    pub fn get_index_count(&self) -> u32 {
        self.index_count as u32
    }

    pub fn render(&self) -> Result<(), MeshError> {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null()
            );
        }
        check_opengl_error("Mesh::render")?;
        Ok(())
    }
}

impl TryFrom<Buffer> for VAO {
    type Error = MeshError;
    fn try_from(buffer: Buffer) -> Result<Self, Self::Error> {
        let vbos = buffer.create_vbos()?;
        let vao = match load_vao(&vbos) {
            Ok(vao) => vao,
            Err(e) => {
                delete_vbos(vbos);
                return Err(MeshError::from(e));
            }
        };
        Ok(Self {
            vao: vao,
            vbos: vbos,
            index_count: buffer.get_index_count() as GLuint
        })
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        delete_vbos(self.vbos);
        match check_opengl_error("gl::DeleteBuffers") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
        delete_vao(self.vao);
        match check_opengl_error("gl::DeleteVertexArrays") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}

fn load_vao(vbos: &[GLuint; 4]) -> Result<GLuint, OpenglError> {
    let mut vao: GLuint = 0;

    unsafe { gl::GenVertexArrays(1, &mut vao); }
    check_opengl_error("gl::GenVertexArrays")?;

    unsafe { gl::BindVertexArray(vao); }
    match check_opengl_error("gl::BindVertexArray") {
        Ok(_) => {},
        Err(e) => {
            delete_vao(vao);
            return Err(e);
        }
    }

    for (index, vbo) in vbos[..3].iter().enumerate() {
        match assign_buffer_to_vao(*vbo, index as GLuint, 3, gl::FLOAT) {
            Ok(_) => {},
            Err(e) => {
                delete_vao(vao);
                return Err(e);
            }
        }
    }

    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[3]); }
    match check_opengl_error("gl::BindBuffer") {
        Ok(_) => {},
        Err(e) => {
            delete_vao(vao);
            return Err(e);
        }
    }

    unsafe { gl::BindVertexArray(0); }
    match check_opengl_error("gl::BindVertexArray") {
        Ok(_) => {},
        Err(e) => {
            delete_vao(vao);
            return Err(e);
        }
    }

    for i in 0..3 {
        unsafe { gl::DisableVertexAttribArray(i) }
        match check_opengl_error("gl::DisableVertexAttribArray") {
            Ok(_) => {},
            Err(e) => {
                delete_vao(vao);
                return Err(e);
            }
        }
    }
    Ok(vao)
}

fn assign_buffer_to_vao(vbo: GLuint, index: GLuint, size: GLint, data_type: GLenum) -> Result<(), OpenglError> {
    unsafe {
        gl::EnableVertexAttribArray(index);
        check_opengl_error("gl::EnableVertexAttribArray")?;
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        check_opengl_error("gl::BindBuffer")?;
        gl::VertexAttribPointer(index, size, data_type, gl::FALSE, 0, ptr::null());
        check_opengl_error("gl::VertexAttribPointer")?;    
    }
    Ok(())
}

fn delete_vbos(buffers: [GLuint; 4]) {
    unsafe { gl::DeleteBuffers(4, &buffers[0] as * const GLuint); }
}

fn delete_vao(vao: GLuint) {
    unsafe { gl::DeleteVertexArrays(1, &vao as * const GLuint); }
}

