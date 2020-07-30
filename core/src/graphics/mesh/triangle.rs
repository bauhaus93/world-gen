use std::fmt;

use super::Vertex;
use crate::Point3f;

#[derive(Copy, Clone)]
pub struct Triangle {
  vertex: [Vertex; 3],
}

fn calculate_normal(vertices: &[Vertex; 3], index: usize) -> Point3f {
    let vec_a = vertices[(index + 1) % 3].get_pos() - vertices[index].get_pos();
    let vec_b = vertices[(index + 2) % 3].get_pos() - vertices[index].get_pos();
    vec_a.cross(&vec_b).as_normalized()
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        Self {
            vertex: vertices,
        }
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

    pub fn update_normals(&mut self) {
        for i in 0..3 {
            self.vertex[i].set_normal(calculate_normal(&self.vertex, i));
        }
    }

    pub fn as_vertices(&self) -> &[Vertex; 3] {
        &self.vertex
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex: [Vertex::default(); 3],
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "vertices: {}, {}, {}", self.vertex[0], self.vertex[1], self.vertex[2])
    }
}
