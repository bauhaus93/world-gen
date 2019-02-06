use std::{ ptr, io, ffi::c_void, mem::size_of };
use std::collections::btree_map::{ BTreeMap, Entry };
use gl::types::{ GLuint, GLsizeiptr, GLenum };

use crate::graphics::{ check_opengl_error, OpenglError, mesh::{ Vertex, Triangle } };
use crate::utility::Float;

pub struct Buffer {
    position: Vec<Float>,
    uv: Vec<Float>,
    normal: Vec<Float>,
    index: Vec<GLuint>
}

impl Buffer {

    pub fn is_empty(&self) -> bool {
        self.index.len() == 0
    }

    pub fn get_index_count(&self) -> usize {
        self.index.len()
    }

    pub fn create_vbos(&self) -> Result<[GLuint; 4], OpenglError> {
        let mut vbos: [GLuint; 4] = [0; 4];
        
        unsafe { gl::GenBuffers(4, &mut vbos[0] as * mut GLuint) };
        check_opengl_error("gl::GenBuffers")?;

        match fill_buffer(vbos[0], gl::ARRAY_BUFFER, (self.position.len() * size_of::<Float>()) as GLsizeiptr, self.position.as_ptr() as * const _) {
            Ok(_) => {},
            Err(e) => {
                delete_vbos(vbos);
                return Err(e);
            }
        }

        match fill_buffer(vbos[1], gl::ARRAY_BUFFER, (self.uv.len() * size_of::<Float>()) as GLsizeiptr, self.uv.as_ptr() as * const _) {
            Ok(_) => {},
            Err(e) => {
                delete_vbos(vbos);
                return Err(e);
            }
        }

        match fill_buffer(vbos[2], gl::ARRAY_BUFFER, (self.normal.len() * size_of::<Float>()) as GLsizeiptr, self.normal.as_ptr() as * const _) {
            Ok(_) => {},
            Err(e) => {
                delete_vbos(vbos);
                return Err(e);
            }
        }

        match fill_buffer(vbos[3], gl::ELEMENT_ARRAY_BUFFER, (self.index.len() * size_of::<GLuint>()) as GLsizeiptr, self.index.as_ptr() as * const _) {
            Ok(_) => {},
            Err(e) => {
                delete_vbos(vbos);
                return Err(e);
            }
        }
        Ok(vbos)
    }
}

impl From<&[Triangle]> for Buffer {
    fn from(triangles: &[Triangle]) -> Self {
        let mut indexed_vertices: BTreeMap<Vertex, GLuint> = BTreeMap::new();
        let mut position_buffer: Vec<Float> = Vec::new();
        let mut uv_buffer: Vec<Float> = Vec::new();
        let mut normal_buffer: Vec<Float> = Vec::new();
        let mut index_buffer: Vec<GLuint> = Vec::new();
        for triangle in triangles.iter() {
            for vertex in triangle.as_vertices() {
                match indexed_vertices.entry(*vertex) {
                    Entry::Occupied(o) => {
                        index_buffer.push(*o.get());
                    },
                    Entry::Vacant(v) => {
                        debug_assert!(position_buffer.len() % 3 == 0);
                        debug_assert!(uv_buffer.len() % 3 == 0);
                        debug_assert!(normal_buffer.len() % 3 == 0);
                        let new_index = (position_buffer.len() / 3) as GLuint;
                        position_buffer.extend(vertex.get_pos().as_array());
                        uv_buffer.extend(vertex.get_uv().as_array());
                        normal_buffer.extend(vertex.get_normal().as_array());
                        index_buffer.push(new_index);
                        v.insert(new_index);
                    }
                }
            }
        }
        Self {
            position: position_buffer,
            uv: uv_buffer,
            normal: normal_buffer,
            index: index_buffer
        }
    }
}

impl From<Vec<Triangle>> for Buffer {
    fn from(triangles: Vec<Triangle>) -> Self {
        Self::from(&triangles[..])
    }
}

fn fill_buffer(buffer_id: GLuint, buffer_type: GLenum, buffer_size: GLsizeiptr, buffer_data: * const c_void) -> Result<(), OpenglError> {
    unsafe {
        gl::BindBuffer(buffer_type, buffer_id);
        check_opengl_error("gl::BindBuffer")?;
        gl::BufferData(buffer_type, buffer_size, buffer_data, gl::STATIC_DRAW);
        check_opengl_error("gl::BufferData")?;
    }
    Ok(()) 
}

fn delete_vbos(buffers: [GLuint; 4]) {
    unsafe { gl::DeleteBuffers(4, &buffers[0] as * const GLuint); }
}