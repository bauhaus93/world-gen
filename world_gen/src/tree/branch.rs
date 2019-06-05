use std::ops::Add;

use rand::Rng;
use glm::{ Vector3, GenNum, normalize };

use graphics::mesh::{ Vertex, Triangle, VertexBuffer };
use utility::Float;
use super::part::Part;

#[allow(dead_code)]
pub struct Branch {
    origin: Vector3<Float>,
    parts: Vec<Part>,
    sub_branches: Vec<Branch>
}

#[allow(dead_code)]
fn create_parts<R: Rng + ?Sized>(part_count: u32, variation: Float, initial_dir: Vector3<Float>, initial_radius: Float, rng: &mut R) -> Vec<Part> {
    debug_assert!(part_count > 1);
    let mut parts = Vec::new();
    let mut direction = initial_dir;
    let mut radius = initial_radius;
    for i in 0..part_count {
        let variation_vec = Vector3::new(rng.gen_range(-variation, variation),
                                         rng.gen_range(-variation, variation),
                                         0.);
        let length = if i + 1 == part_count {
            rng.gen_range(0.1, 0.3)
        } else {
            rng.gen_range(0.2, 1.)
        };
        direction = normalize(direction.add(variation_vec));
        parts.push(Part::new(direction, radius, length));
        if i + 2 == part_count {
            radius = 0.;
        } else {
            radius *= 0.9;
        }
    }
    parts
}

#[allow(dead_code)]
impl Branch {
    pub fn new<R: Rng + ?Sized>(origin: Vector3<Float>,
                                part_count: u32,
                                variation: Float,
                                initial_dir: Vector3<Float>,
                                initial_radius: Float,
                                sub_branch_count: u32,
                                rng: &mut R) -> Self {
        let parts = create_parts(part_count, variation, initial_dir, initial_radius, rng);
        let sub_branches = Vec::new();
        let mut branch = Self {
            origin: origin,
            parts: parts,
            sub_branches: sub_branches
        };
        for _ in 0..sub_branch_count {
            branch.add_sub_branch(part_count / 2, variation, rng);
        }
        branch
    }

    pub fn set_origin(&mut self, new_origin: Vector3<Float>) {
        self.origin = new_origin;
    }

    fn add_sub_branch<R: Rng + ?Sized>(&mut self, part_count: u32, variation: Float, rng: &mut R) {
        let mut origin = self.origin; 
        let part_index = rng.gen_range(0, self.parts.len());
        self.parts.iter()
            .take(part_index)
            .for_each(|p| origin = p.next_origin(origin));
        let offset_factor = rng.gen_range(0., 1.);
        let ancestor_part = &self.parts[part_index];
        origin = ancestor_part.next_origin_factored(origin, offset_factor);
        let initial_variation = Vector3::new(rng.gen_range(-variation, variation),
                                             rng.gen_range(-variation, variation),
                                             0.);
        let sub_dir = normalize(
            ancestor_part.create_sub_dir(rng.gen_range(0., 360f32.to_radians()))
            .add(initial_variation)
        );
        let sub_radius = ancestor_part.get_radius() * rng.gen_range(0.6, 0.8);
        let sub = Branch::new(origin, part_count, variation, sub_dir, sub_radius, 0, rng);
        self.sub_branches.push(sub);
    }

    fn build_triangles(&self, point_count: u32) -> Vec<Triangle> {
        debug_assert!(point_count >= 3);
        debug_assert!(self.parts.len() > 1);
        let ring_template = match self.parts.first() {
            Some(part) => part.create_ring_template(point_count),
            None => unreachable!()
        };
        let mut triangles = Vec::new();
        let mut prev_ring: Option<Vec<Vector3<Float>>> = None;
        let mut origin = self.origin;
        for part in self.parts.iter() {
            origin = part.next_origin(origin);
            let top_ring = part.align_ring(origin, ring_template.as_slice());
            let new_triangles = match prev_ring {
                Some(bottom_ring) => {
                    if part.is_peak() {
                        self.create_peak_triangles(bottom_ring.as_slice(), origin)
                    } else {
                        self.create_triangles(bottom_ring.as_slice(), top_ring.as_slice())
                    }
                },
                None => {
                    let bottom_ring = part.align_ring(self.origin, ring_template.as_slice());
                    self.create_triangles(bottom_ring.as_slice(), top_ring.as_slice())
                }
            };
            triangles.extend(new_triangles);

            prev_ring = Some(top_ring);
        }
        for sub in self.sub_branches.iter() {
            triangles.extend(sub.build_triangles(point_count));
        }
        triangles
    }

    pub fn build_buffer(&self, point_count: u32) -> VertexBuffer {
        VertexBuffer::from(self.build_triangles(point_count).as_slice())
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

    fn create_peak_triangles(&self, bottom_ring: &[Vector3<Float>], next_origin: Vector3<Float>) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let bottom_iter = bottom_ring.iter().zip(bottom_ring.iter().cycle().skip(1));
        let mut vertex_top = Vertex::default();
        vertex_top.set_pos(next_origin);
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
    }
}

