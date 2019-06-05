use std::collections::btree_map::{ BTreeMap, Entry };
use gl::types::{ GLint, GLuint, GLenum };

use crate::utility::Float;
use crate::mesh::{ Triangle, Vertex, VAO, VAOBuilder, MeshError };

pub fn create_vao_from_triangles(triangles: &[Triangle]) -> Result<VAO, MeshError> {
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
                    normal_buffer.extend(triangle.get_normal().as_array());
                    index_buffer.push(new_index);
                    v.insert(new_index);
                }
            }
        }
    }
    let vao = VAOBuilder::new()
        .set_triangle()
        .add_float_buffer(&position_buffer, 0, 3)
        .add_float_buffer(&uv_buffer, 1, 3)
        .add_float_buffer(&normal_buffer, 2, 3)
        .add_index_buffer(&index_buffer)
        .finish()?;
    Ok(vao)
}

