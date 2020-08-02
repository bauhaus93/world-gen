use std::fmt;

use super::Vertex;
use crate::{Point2f, Point3f};

#[derive(Copy, Clone)]
pub struct Triangle {
    vertex: [Vertex; 3],
    triangle_normal: Point3f,
}

fn calculate_normal(vertices: &[Vertex; 3], index: usize) -> Point3f {
    let vec_a = vertices[(index + 1) % 3].get_pos() - vertices[index].get_pos();
    let vec_b = vertices[(index + 2) % 3].get_pos() - vertices[index].get_pos();
    vec_a.cross(&vec_b).as_normalized()
}

impl Triangle {
    pub fn new(vertices: [Vertex; 3]) -> Self {
        let normal = calculate_normal(&vertices, 0);
        Self {
            vertex: vertices,
            triangle_normal: normal,
        }
    }

    pub fn set_vertex(&mut self, vertex: Vertex, index: usize) {
        debug_assert!(index < 3);
        self.vertex[index] = vertex;
    }

    pub fn set_uv_layer(&mut self, uv_layer: u32) {
        self.vertex
            .iter_mut()
            .for_each(|v| v.set_uv_layer(uv_layer));
    }

    pub fn get_uv_dim(&self) -> u8 {
        debug_assert!(self.vertex[0].get_uv_dim() == self.vertex[1].get_uv_dim());
        debug_assert!(self.vertex[1].get_uv_dim() == self.vertex[2].get_uv_dim());
        self.vertex[0].get_uv_dim()
    }

    pub fn update_triangle_normal(&mut self) {
        self.triangle_normal = calculate_normal(&self.vertex, 0);
    }

    pub fn update_vertex_normals(&mut self) {
        for i in 0..3 {
            self.vertex[i].set_normal(self.triangle_normal);
        }
    }

    pub fn force_ccw(&mut self) {
        if !has_winding_order_ccw(&[
            self.vertex[0].get_pos().as_xy(),
            self.vertex[1].get_pos().as_xy(),
            self.vertex[2].get_pos().as_xy(),
        ]) {
            self.vertex.swap(0, 1);
        }
    }

    pub fn as_vertices(&self) -> &[Vertex; 3] {
        &self.vertex
    }
    pub fn get_vertices_mut(&mut self) -> &mut [Vertex; 3] {
        &mut self.vertex
    }
}

fn has_winding_order_ccw(points: &[Point2f; 3]) -> bool {
    let det = (points[1][0] - points[0][0]) * (points[2][1] - points[0][1])
        - (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]);
    det > 0. // Don't know if correct
}

impl Default for Triangle {
    fn default() -> Self {
        Self {
            vertex: [Vertex::default(); 3],
            triangle_normal: Point3f::new(0., 0., 1.)
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "vertices: {}, {}, {}",
            self.vertex[0], self.vertex[1], self.vertex[2]
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const RANDOM_SEED: u64 = 9001;

    #[test]
    fn test_determine_winding_order_ccw() {
        let points = [
            Point2f::new(0., 1.),
            Point2f::new(0., 0.),
            Point2f::new(1., 1.),
        ];
        assert!(has_winding_order_ccw(&points));
    }

    #[test]
    fn test_determine_winding_order_cw() {
        let points = [
            Point2f::new(0., 0.),
            Point2f::new(0., 1.),
            Point2f::new(1., 1.),
        ];
        assert!(!has_winding_order_ccw(&points));
    }
}
