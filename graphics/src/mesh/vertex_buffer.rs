use std::{ ffi::c_void, mem::size_of };
use std::collections::btree_map::{ BTreeMap, Entry };
use std::convert::TryInto;

use gl;
use gl::types::{ GLuint, GLint, GLenum, GLsizeiptr };

use utility::Float;
use crate::{ check_opengl_error, OpenglError };
use crate::mesh::utility::{ delete_vao, delete_vbos };
use super::{ VAO, Triangle, Vertex, MeshError };

pub const BUFFER_POSTION: u8 = 1 << 1;
pub const BUFFER_UV: u8 = 1 << 2;
pub const BUFFER_NORMAL: u8 = 1 << 3;

pub struct VertexBuffer {
    buffer_list: Vec<Buffer>,
    index_list: Vec<GLuint>,
    element_type: GLenum
}

enum Buffer {
    Float { data: Vec<Float>, attribute_index: GLuint, element_count: GLint }
}

impl VertexBuffer {
    pub fn add_float_buffer(&mut self, buffer_data: Vec<Float>, attribute_index: u32, element_count: u32) {
        let buffer = Buffer::Float {
            data: buffer_data,
            attribute_index: attribute_index.try_into().unwrap(),
            element_count: element_count.try_into().unwrap()
        };
        self.buffer_list.push(buffer);
    }

    pub fn set_index_buffer(&mut self, index_data: Vec<GLuint>) {
        self.index_list = index_data;
    }
   
}

impl Default for VertexBuffer {
    fn default() -> VertexBuffer {
        VertexBuffer {
            buffer_list: Vec::new(),
            index_list: Vec::new(),
            element_type: gl::TRIANGLES
        }
    }
}

impl TryInto<VAO> for VertexBuffer {
    type Error = MeshError;

    fn try_into(self) -> Result<VAO, Self::Error> {
        let vbo_ids = create_vbos(&self.index_list, &self.buffer_list)?;
        let vao_id = match create_vao(&vbo_ids, &self.buffer_list) {
            Ok(id) => id,
            Err(e) => {
                if let Err(new_err) = delete_vbos(&vbo_ids) {
                    error!("Additional error: {}", new_err);
                }
                return Err(e.into());
            }
        };

        let vao = VAO::new(
            vao_id,
            &vbo_ids,
            self.element_type,
            self.index_list.len().try_into().unwrap()
        );
        Ok(vao)
    }
}

pub fn triangles_to_buffers(triangles: &[Triangle], buffer_flags: u8) ->
    (Vec<Float>, Vec<Float>, Vec<Float>, Vec<GLuint>) {

    let uv_size = triangles[0].get_uv_dim();
    let mut indexed_vertices: BTreeMap<Vertex, GLuint> = BTreeMap::new();
    let mut position_buffer: Vec<Float> = Vec::new();
    let mut uv_buffer: Vec<Float> = Vec::new();
    let mut normal_buffer: Vec<Float> = Vec::new();
    let mut index_buffer: Vec<GLuint> = Vec::new();
    for triangle in triangles.iter() {
        debug_assert!(triangle.get_uv_dim() == uv_size);
        for vertex in triangle.as_vertices() {
            match indexed_vertices.entry(*vertex) {
                Entry::Occupied(o) => {
                    index_buffer.push(*o.get());
                },
                Entry::Vacant(v) => {
                    debug_assert!(position_buffer.len() % 3 == 0);
                    debug_assert!(uv_buffer.len() % (uv_size as usize) == 0);
                    debug_assert!(normal_buffer.len() % 3 == 0);
                    let new_index = (position_buffer.len() / 3) as GLuint;
                    if buffer_flags & BUFFER_POSTION != 0 {
                        position_buffer.extend(vertex.get_pos().as_array());
                    }
                    if buffer_flags & BUFFER_UV != 0  {
                        match uv_size {
                            2 => {
                                uv_buffer.extend(vertex.get_uv().as_array());
                            },
                            3 => {
                                uv_buffer.extend(vertex.get_uv_layered().as_array());
                            },
                            unknown_dim => {
                                panic!("Unknown uv dimension: {}", unknown_dim);
                            }
                        }
                    }
                    if buffer_flags & BUFFER_NORMAL != 0 {
                        normal_buffer.extend(triangle.get_normal().as_array());
                    }
                    index_buffer.push(new_index);
                    v.insert(new_index);
                }
            }
        }
    }
    (position_buffer, uv_buffer, normal_buffer, index_buffer)
}

impl From<&[Triangle]> for VertexBuffer {
    fn from(triangles: &[Triangle]) -> VertexBuffer {
        let uv_size = triangles[0].get_uv_dim();
        let (position_buffer, uv_buffer, normal_buffer, index_buffer) =
            triangles_to_buffers(triangles, BUFFER_POSTION | BUFFER_UV | BUFFER_NORMAL);
        let mut buffer = VertexBuffer::default();

        if position_buffer.len() > 0 {
            buffer.add_float_buffer(position_buffer, 0, 3);
        }
        if uv_buffer.len() > 0 {
            buffer.add_float_buffer(uv_buffer, 1, uv_size as u32);
        }
        if normal_buffer.len() > 0 {
            buffer.add_float_buffer(normal_buffer, 2, 3);
        }
        buffer.set_index_buffer(index_buffer);

        buffer
    }
}

fn create_vao(vbos: &[GLuint], buffer_list: &[Buffer]) -> Result<GLuint, OpenglError> {
    debug_assert!(vbos.len() == 1 + buffer_list.len()); // 1+ because vbo @ 0 is index vbo
    debug_assert!(vbos.iter().all(|v| *v != 0));
    let mut vao: GLuint = 0;

    unsafe { gl::GenVertexArrays(1, &mut vao); }
    check_opengl_error("gl::GenVertexArrays")?;

    unsafe { gl::BindVertexArray(vao); }
    match check_opengl_error("gl::BindVertexArray") {
        Ok(_) => {},
        Err(e) => {
            if let Err(new_err) = delete_vao(vao) {
                error!("Additional error: {}", new_err);
            }
            return Err(e);
        }
    }

    for (vbo, buffer) in vbos.iter().skip(1).zip(buffer_list.iter()) {
        match buffer {
            Buffer::Float { attribute_index, element_count, .. } => {
                let result = assign_buffer_to_vao(
                    *vbo,
                    *attribute_index,
                    *element_count,
                    gl::FLOAT
                );
                if let Err(e) = result {
                    if let Err(new_err) = delete_vao(vao) {
                        error!("Additional error: {}", new_err);
                    }
                    return Err(e);
                }
            }
        }
    }

    unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, vbos[0]); }

    if let Err(e) = check_opengl_error("gl::BindBuffer") {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
        return Err(e);
    }

    if let Err(e) = disable_vao(vao) {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
        return Err(e);
    }

    if let Err(e) = disable_vertex_attributes(vbos.len()) {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
        return Err(e);
    }

    Ok(vao)
}

fn disable_vao(vao: GLuint) -> Result<(), OpenglError> {
    unsafe { gl::BindVertexArray(0); }
    if let Err(e) = check_opengl_error("gl::BindVertexArray") {
        if let Err(new_err) = delete_vao(vao) {
            error!("Additional error: {}", new_err);
        }
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

fn create_vbos(index_list: &[GLuint], buffer_list: &[Buffer]) -> Result<Vec<GLuint>, OpenglError> {
    let vbos = create_vbo_ids(buffer_list.len() + 1)?;

    for (buffer, vbo) in buffer_list.iter().zip(vbos.iter().skip(1)) {
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
            }
        }
    }

    let buffer_size = index_list.len() * size_of::<GLuint>();
    let result = fill_vbo(
        vbos[0],
        gl::ELEMENT_ARRAY_BUFFER,
        buffer_size.try_into().unwrap(),
        index_list.as_ptr() as * const _
    );
    if let Err(e) = result {
        if let Err(new_err) = delete_vbos(&vbos) {
            error!("Additional error: {}", new_err);
        }
        return Err(e)
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