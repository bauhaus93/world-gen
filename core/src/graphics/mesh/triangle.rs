use std::fmt;

use glm::{ Vector3, GenNum, cross, normalize };

use crate::Float;
use super::Vertex;

#[derive(Copy, Clone)]
pub struct Triangle {
  vertex: [Vertex; 3],
  normal: Vector3<Float>
}

fn calculate_normal(vertices: &[Vertex; 3]) -> Vector3<Float> {
    let vec_a = vertices[1].get_pos() - vertices[0].get_pos();
    let vec_b = vertices[2].get_pos() - vertices[0].get_pos();
    normalize(cross(vec_a, vec_b))
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        let normal = calculate_normal(&vertices);
        Self {
            vertex: vertices,
            normal: normal
        }
    }

    pub fn get_normal(&self) -> Vector3<Float> {
        self.normal
    }

    pub fn set_vertex(&mut self, vertex: Vertex, index: usize) {
        debug_assert!(index < 3);
        self.vertex[index] = vertex;
    }

    pub fn set_uv_layer(&mut self, uv_layer: u32) {
        self.vertex.iter_mut().for_each(|v| v.set_uv_layer(uv_layer));
    }

    pub fn get_uv_dim(&self) -> u8 {
        debug_assert!(self.vertex[0].get_uv_dim() == self.vertex[1].get_uv_dim());
        debug_assert!(self.vertex[1].get_uv_dim() == self.vertex[2].get_uv_dim());
        self.vertex[0].get_uv_dim()
    }

    pub fn update_normal(&mut self) {
        self.normal = calculate_normal(&self.vertex);
    }

    pub fn as_vertices(&self) -> &[Vertex; 3] {
        &self.vertex
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex: [Vertex::default(); 3],
            normal: Vector3::from_s(0.)
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "normal = {:.2}/{:.2}/{:.2}, vertices: {}, {}, {}", self.normal[0], self.normal[1], self.normal[2], self.vertex[0], self.vertex[1], self.vertex[2])
    }
}
