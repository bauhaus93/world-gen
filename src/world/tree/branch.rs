use std::ops::Add;

use rand::Rng;
use glm::{ Vector3, GenNum, normalize };

use crate::graphics::mesh::{ Vertex, Triangle, Buffer };
use crate::utility::Float;
use super::part::Part;

pub struct Branch {
    parts: Vec<Part>
}

impl Branch {
    pub fn new<R: Rng + ?Sized>(variation: Float, initial_dir: Vector3<Float>, rng: &mut R) -> Self {
        let mut parts = Vec::new();
        let part_count = rng.gen_range(10, 20);
        let mut direction = initial_dir;
        let mut radius = 1.;
        for i in 0..part_count {
            let variation_vec = Vector3::new(rng.gen_range(-variation, variation),
                                            rng.gen_range(-variation, variation),
                                            0.);
            let length = rng.gen_range(0.2, 1.);
            direction = normalize(direction.add(variation_vec));
            parts.push(Part::new(direction, radius, length));
            radius *= 0.9;
        }
        Self {
            parts: parts
        }
    }

    pub fn build_buffer(&self, point_count: u32) -> Buffer {
        debug_assert!(point_count >= 3);
        let ring_template = match self.parts.first() {
            Some(part) => part.create_ring_template(point_count),
            None => unreachable!()
        };
        let mut triangles = Vec::new();
        let mut prev_ring: Option<Vec<Vector3<Float>>> = None;
        let mut origin = Vector3::from_s(0.);
        for part in self.parts.iter() {
            origin = part.next_origin(origin);
            let top_ring = part.align_ring(origin, ring_template.as_slice());
            let new_triangles = match prev_ring {
                Some(bottom_ring) => {
                    self.create_triangles(bottom_ring.as_slice(), top_ring.as_slice())
                },
                None => {
                    self.create_triangles(ring_template.as_slice(), top_ring.as_slice())
                }
            };
            triangles.extend(new_triangles);
            prev_ring = Some(top_ring);
        }
        Buffer::from(triangles.as_slice())
    }

    fn create_triangles(&self, bottom_ring: &[Vector3<Float>], top_ring: &[Vector3<Float>]) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let bottom_iter = bottom_ring.iter().zip(bottom_ring.iter().cycle().skip(1));
        let top_iter = top_ring.iter().zip(top_ring.iter().cycle().skip(1));
        for ((a, b), (c, d)) in bottom_iter.zip(top_iter) {
            let mut vertex_a = Vertex::default();
            vertex_a.set_pos(*a);
            vertex_a.set_uv(Vector3::from_s(0.));

            let mut vertex_b = Vertex::default();
            vertex_b.set_pos(*b);
            vertex_b.set_uv(Vector3::new(1., 0., 0.));

            let mut vertex_c = Vertex::default();
            vertex_c.set_pos(*c);
            vertex_c.set_uv(Vector3::new(0., 1., 0.));

            let mut vertex_d = Vertex::default();
            vertex_d.set_pos(*d);
            vertex_d.set_uv(Vector3::new(1., 1., 0.));
            
            let mut triangle = Triangle::default();
            triangle.set_vertex(vertex_a, 0);
            triangle.set_vertex(vertex_b, 1);
            triangle.set_vertex(vertex_c, 2);
            triangle.update_normal();
            triangle.set_uv_layer(1);
            triangles.push(triangle);

            let mut triangle = Triangle::default();
            triangle.set_vertex(vertex_b, 0);
            triangle.set_vertex(vertex_d, 1);
            triangle.set_vertex(vertex_c, 2);
            triangle.update_normal();
            triangles.push(triangle);
        }
        triangles
    } 

/*    fn create_peak_triangles(&self, bottom_ring: &[Vector3<Float>], top_part: &Part) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let bottom_iter = bottom_ring.iter().zip(bottom_ring.iter().cycle().skip(1));
        let mut vertex_top = Vertex::default();
        vertex_top.set_pos(top_part.get_anchor().add(top_part.get_direction()));
        vertex_top.set_uv(Vector3::new(0., 1., 0.));
        for (a, b) in bottom_iter {
            let mut vertex_a = Vertex::default();
            vertex_a.set_pos(*a);
            vertex_a.set_uv(Vector3::from_s(0.));

            let mut vertex_b = Vertex::default();
            vertex_b.set_pos(*b);
            vertex_b.set_uv(Vector3::new(1., 0., 0.));

            let mut triangle = Triangle::default();
            triangle.set_vertex(vertex_a, 0);
            triangle.set_vertex(vertex_b, 1);
            triangle.set_vertex(vertex_top, 2);
            triangle.update_normal();
            triangle.set_uv_layer(1);
            triangles.push(triangle);
        }
        triangles
    }*/
}

