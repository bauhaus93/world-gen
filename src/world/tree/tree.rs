use std;
use std::ops::{ Add, Sub };
use std::convert::TryFrom;

use num_traits::One;
use rand;
use rand::Rng;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use glm::{ Vector3, Vector4, Matrix4, GenNum, normalize, cross, ext::rotate };

use crate::utility::Float;
use crate::graphics::mesh::{ Vertex, Triangle, Buffer, Mesh };
use crate::graphics::create_translation_matrix;
use crate::world::chunk::ChunkError;

pub struct Tree {
    radius: Float,
    length: Float,
    root: Branch
}

struct Branch {
    anchor: Vector3<Float>,
    direction: Vector3<Float>,
    radius_factor: Float,
    length_factor: Float,
    next_part: Option<Box<Branch>>
}

impl Branch {
    pub fn new<R: Rng + ?Sized>(ancestor_dir: Vector3<Float>, rng: &mut R) -> Self {
        let mut branch = Self {
            anchor: Vector3::from_s(0.),
            direction: normalize(ancestor_dir.add(Vector3::new(rng.gen_range(-0.2, 0.2),
                                                               rng.gen_range(-0.2, 0.2),
                                                               0.))),
            radius_factor: rng.gen_range(0.9, 1.),
            length_factor: rng.gen_range(0.1, 1.),
            next_part: None
        };
        let extensions = rng.gen_range(10, 20);
        info!("Created root branch: anchor = {:?}, dir = {:?}, r = {}, l = {}, ext = {}", branch.anchor, branch.direction, branch.radius_factor, branch.length_factor, extensions);
        branch.grow(extensions, rng);
        branch
    }

    pub fn build_buffer(&self, point_count: u8) -> Buffer {
        debug_assert!(point_count >= 3);
        let ring = self.calculate_ring(point_count);
        Buffer::from(self.collect_triangles(ring.as_slice(), 1.))
    }

    fn grow<R: Rng + ?Sized>(&mut self, extension_count: i32, rng: &mut R) {
        let radius_factor = if extension_count == 0 {
            0.
        } else if extension_count < 3 {
            rng.gen_range(0.5 * self.radius_factor, 0.7 * self.radius_factor)
        }  else {
            rng.gen_range(0.9 * self.radius_factor, 0.95 * self.radius_factor)
        };
        let length_factor = if extension_count == 0 {
            rng.gen_range(0.1, 0.2)
        } else {
            rng.gen_range(0.1, 1.)
        };
        let mut branch = Self {
            anchor: self.anchor.add(self.direction * self.length_factor),
            direction: normalize(self.direction.add(Vector3::new(rng.gen_range(-0.2, 0.2),
                                                                 rng.gen_range(-0.2, 0.2),
                                                                 0.))),
            radius_factor: radius_factor,
            length_factor: length_factor,
            next_part: None
        };
        
        info!("Created branch: anchor = {:?}, dir = {:?}, r = {}, l = {}", branch.anchor, branch.direction, branch.radius_factor, branch.length_factor);
        if extension_count > 0 {
            branch.grow(extension_count - 1, rng);
        }
        self.next_part = Some(Box::new(branch))
    }

    fn calculate_ring(&self, count: u8) -> Vec<Vector3<Float>> {
        let right = normalize(cross(self.direction, Vector3::new(0., 0., 1.)));
        let p_base = self.anchor.add(right).extend(1.);
        let one = Matrix4::<Float>::one();
        let mut points = Vec::new();
        for i in 0..count {
            let rot_mat = rotate(&one, ((360. / (count as f32)) * i as f32).to_radians(), self.direction);
            let p = (rot_mat * p_base).truncate(3);
            points.push(p);
        }
        points
    }

    fn collect_triangles(&self, ring: &[Vector3<Float>], prev_radius_factor: Float) -> Vec<Triangle> {
        let bottom_points: Vec<Vector3<Float>> = ring.iter().map(|p| (*p * prev_radius_factor).add(self.anchor)).collect();
        if let Some(ref next_branch) = self.next_part {
            let top_points: Vec<Vector3<Float>> = ring.iter().map(|p| (*p * self.radius_factor).add(self.anchor).add(self.direction * self.length_factor)).collect();
            let mut triangles = self.create_triangles(bottom_points.as_slice(), top_points.as_slice());
            triangles.extend(next_branch.collect_triangles(ring, self.radius_factor));
            triangles
        } else {
            self.create_peak_triangles(bottom_points.as_slice())
        }
    }
    
    fn create_triangles(&self, bottom_points: &[Vector3<Float>], top_points: &[Vector3<Float>]) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let bottom_iter = bottom_points.iter().zip(bottom_points.iter().cycle().skip(1));
        let top_iter = top_points.iter().zip(top_points.iter().cycle().skip(1));
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

    fn create_peak_triangles(&self, bottom_points: &[Vector3<Float>]) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        let bottom_iter = bottom_points.iter().zip(bottom_points.iter().cycle().skip(1));
        let mut vertex_top = Vertex::default();
        vertex_top.set_pos(self.anchor.add(self.direction));
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

impl Tree {
    pub fn new() -> Self {
        let mut rng = SmallRng::from_seed([0; 16]);
        Self {
            radius: rng.gen_range(1., 3.),
            length: rng.gen_range(10., 40.),
            root: Branch::new(Vector3::new(0., 0., 1.), &mut rng)
        }
    }

    pub fn build_mesh(&self) -> Result<Mesh, ChunkError> {
        let buffer = self.root.build_buffer(16);
        let mesh = Mesh::try_from(buffer)?;
        Ok(mesh)
    }
}
