use std::{ ffi::c_void, mem::size_of };
use std::convert::TryInto;
use gl;
use gl::types::{ GLint, GLuint, GLenum, GLsizeiptr };

use crate::{ check_opengl_error, OpenglError };
use utility::Float;
use crate::mesh::utility:: { delete_vao, delete_vbos };
use super::VAO;

pub enum Buffer {
    Float { data: Vec<Float>, attribute_index: GLuint, tuple_size: GLint },
    Index { data: Vec<GLuint> }
}

pub struct VAOBuilder {
    buffer_list: Vec<Buffer>,
    element_type: GLenum,
    index_count: GLuint
}

impl VAOBuilder {

    pub fn new() -> Self {
        VAOBuilder {
            buffer_list: Vec::new(),
            element_type: gl::TRIANGLES,
            index_count: 0
        }
    }

    pub fn set_triangle(mut self) -> Self {
        self.element_type = gl::TRIANGLES;
        self
    }

    pub fn add_float_buffer(mut self, data: &[Float], attribute_index: u32, tuple_size: u32) -> Self {
        let buffer = Buffer::Float {
            data: Vec::<Float>::from(data),
            attribute_index: attribute_index.try_into().unwrap(),
            tuple_size: tuple_size.try_into().unwrap()
        };
        self.buffer_list.push(buffer);
        
        self
    }
    pub fn add_index_buffer(mut self, data: &[GLuint]) -> Self {
        self.index_count = data.len().try_into().unwrap();
        let buffer = Buffer::Index {
            data: Vec::<GLuint>::from(data)
        };
        self.buffer_list.push(buffer);
        
        self
    }

    pub fn finish(self) -> Result<VAO, OpenglError> {
        let vbo_ids = create_vbos(&self.buffer_list)?;
        let vao_id = match create_vao(&vbo_ids, &self.buffer_list) {
            Ok(id) => id,
            Err(e) => {
                if let Err(new_err) = delete_vbos(&vbo_ids) {
                    error!("Additional error: {}", new_err);
                }
                return Err(e);
            }
        };

        debug_assert!(self.element_type == gl::TRIANGLES);
        let vao = VAO::new(vao_id, &vbo_ids, self.element_type, self.index_count);
        Ok(vao)
    }

}

fn create_vao(vbos: &[GLuint], buffer_list: &[Buffer]) -> Result<GLuint, OpenglError> {
    debug_assert!(vbos.len() == buffer_list.len());
    debug_assert!(vbos.iter().all(|v| *v != 0));
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

    for (vbo, buffer) in vbos.iter().zip(buffer_list.iter()) {
        match buffer {
            Buffer::Float { attribute_index, tuple_size, .. } => {
                let result = assign_buffer_to_vao(
                    *vbo,
                    *attribute_index,
                    *tuple_size,
                    gl::FLOAT
                );

                if let Err(e) = result {
                    if let Err(new_err) = delete_vao(vao) {
                        error!("Additional error: {}", new_err);
                    }
                    return Err(e);
                }
            },
            Buffer::Index { .. } => {
                unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, *vbo); }
                if let Err(e) = check_opengl_error("gl::BindBuffer") {
                    if let Err(new_err) = delete_vao(vao) {
                        error!("Additional error: {}", new_err);
                    }
                    return Err(e);
                }
            }
        }
    }

    if let Err(e) = disable_vao(vao) {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
    }

    if let Err(e) = disable_vertex_attributes(vbos.len()) {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
    }

    Ok(vao)
}

fn disable_vao(vao: GLuint) -> Result<(), OpenglError> {
    unsafe { gl::BindVertexArray(0); }
    if let Err(e) = check_opengl_error("gl::BindVertexArray") {
        delete_vao(vao);
        Err(e)
    } else {
        Ok(())
    }
}

fn disable_vertex_attributes(count: usize) -> Result<(), OpenglError> {
    for i in 0..count {
        unsafe { gl::DisableVertexAttribArray(i.try_into().unwrap()) }
        if let Err(e) = check_opengl_error("gl::DisableVertexAttribArray") {
            return Err(e);
        }
    }
    Ok(())
}

fn assign_buffer_to_vao(vbo: GLuint, index: GLuint, size: GLint, data_type: GLenum) -> Result<(), OpenglError> {
    unsafe {
        gl::EnableVertexAttribArray(index);
        check_opengl_error("gl::EnableVertexAttribArray")?;
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        check_opengl_error("gl::BindBuffer")?;
        gl::VertexAttribPointer(index, size, data_type, gl::FALSE, 0, std::ptr::null());
        check_opengl_error("gl::VertexAttribPointer")?;    
    }
    Ok(())
}

fn create_vbos(buffer_list: &[Buffer]) -> Result<Vec<GLuint>, OpenglError> {
    let vbos = create_vbo_ids(buffer_list.len())?;

    for (buffer, vbo) in buffer_list.iter().zip(vbos.iter()) {
        match buffer {
            Buffer::Float { data, .. } => {
                let buffer_size = data.len() * size_of::<Float>();
                let result = fill_vbo(
                    *vbo,
                    gl::ARRAY_BUFFER,
                    buffer_size.try_into().unwrap(),
                    data.as_ptr() as * const c_void
                );
                if let Err(e) = result {
                    if let Err(new_err) = delete_vbos(&vbos) {
                        error!("Additional error: {}", new_err);
                    }
                    return Err(e);
                }
            },
            Buffer::Index { data } => {
                let buffer_size = data.len() * size_of::<GLuint>();
                let result = fill_vbo(
                    *vbo,
                    gl::ELEMENT_ARRAY_BUFFER,
                    buffer_size.try_into().unwrap(),
                    data.as_ptr() as * const _
                );
                if let Err(e) = result {
                    if let Err(new_err) = delete_vbos(&vbos) {
                        error!("Additional error: {}", new_err);
                    }
                }
            }
        }
    }
    Ok(vbos)
}


fn create_vbo_ids(size: usize) -> Result<Vec<GLuint>, OpenglError> {
    let mut vbos: Vec<GLuint> = Vec::with_capacity(size);
    vbos.resize(size, 0);
    
    unsafe { gl::GenBuffers(size.try_into().unwrap(), vbos.as_mut_ptr() as * mut GLuint) };
    check_opengl_error("gl::GenBuffers")?;

    Ok(vbos)
}

fn fill_vbo(
    buffer_id: GLuint,
    buffer_type: GLenum,
    buffer_size: GLsizeiptr,
    buffer_data: * const c_void) -> Result<(), OpenglError> {
    unsafe {
        gl::BindBuffer(buffer_type, buffer_id);
        check_opengl_error("gl::BindBuffer")?;
        gl::BufferData(buffer_type, buffer_size, buffer_data, gl::STATIC_DRAW);
        check_opengl_error("gl::BufferData")?;
    }
    Ok(()) 
}



