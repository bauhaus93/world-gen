use std::fmt;

use glm::{ Vector3, Matrix4 };

use crate::utility::Float;
use super::Vertex;

#[derive(Copy, Clone)]
pub struct Triangle {
  vertex: [Vertex; 3]
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Triangle {
        Triangle {
            vertex: vertices
        }
    }

    pub fn get_vertices(&self) -> &[Vertex] {
        &self.vertex
    }

    pub fn set_vertex(&mut self, vertex: Vertex, index: usize) {
        debug_assert!(index < 3);
        self.vertex[index] = vertex;
    }

    pub fn set_uv_layer(&mut self, uv_layer: u32) {
        self.vertex.iter_mut().for_each(|v| v.set_uv_layer(uv_layer));
    }

    pub fn into_vertices(self) -> [Vertex; 3] {
        self.vertex
    }

    pub fn as_vertices(&self) -> &[Vertex; 3] {
        &self.vertex
    }

    pub fn get_sorted_vertices(&self) -> [Vertex; 3] {
        let mut sorted_vertices = self.vertex.clone();
        sorted_vertices.sort();
        sorted_vertices
    }

    pub fn on_plane(&self, axis: usize, value: Float) -> bool {
        debug_assert!(axis < 3);
        self.vertex.iter().all(|v| v.on_plane(axis, value))
    }

    pub fn move_vertices(&mut self, offset: Vector3<Float>) {
        self.vertex.iter_mut().for_each(|v| v.move_pos(offset));
    }

    pub fn rotate(&mut self, rotation_matrix: Matrix4<Float>) {
        self.vertex.iter_mut().for_each(|v| v.rotate(rotation_matrix));
    }

    //TODO remove normals from vertices, add normal to triangle
    pub fn get_normal(&self) -> Vector3<Float> {
        self.vertex[0].get_normal()
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex: [Vertex::default(); 3]
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertices: {}, {}, {}", self.vertex[0], self.vertex[1], self.vertex[2])
    }
}