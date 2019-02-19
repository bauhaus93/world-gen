use std::ops::Add;
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
            direction: normalize(ancestor_dir.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                               rng.gen_range(-0.1, 0.1),
                                                               0.))),
            radius_factor: rng.gen_range(0.9, 1.),
            length_factor: rng.gen_range(0.1, 1.),
            next_part: None
        };
        let extensions = rng.gen_range(2, 6);
        info!("Created root branch: anchor = {:?}, dir = {:?}, r = {}, l = {}, ext = {}", branch.anchor, branch.direction, branch.radius_factor, branch.length_factor, extensions);
        branch.extend(extensions, rng);
        branch
    }

    fn extend<R: Rng + ?Sized>(&mut self, extension_count: i32, rng: &mut R) {
        let radius_factor = if extension_count == 0 {
            0.
        } else {
            rng.gen_range(0.9 * self.radius_factor, self.radius_factor)
        };
        let mut branch = Self {
            anchor: self.anchor.add(self.direction),
            direction: normalize(self.direction.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                                 rng.gen_range(-0.1, 0.1),
                                                                 0.))),
            radius_factor: radius_factor,
            length_factor: rng.gen_range(0.1, 1.),
            next_part: None
        };
        
        info!("Created branch: anchor = {:?}, dir = {:?}, r = {}, l = {}", branch.anchor, branch.direction, branch.radius_factor, branch.length_factor);
        if extension_count > 0 {
            branch.extend(extension_count - 1, rng);
        }
        self.next_part = Some(Box::new(branch))
    }

    fn calculate_base_points(&self, count: u8) -> Vec<Vector3<Float>> {
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

    pub fn collect_triangles_by_points(&self, bottom_points: &[Vector3<Float>], top_points: &[Vector3<Float>]) -> Vec<Triangle> {
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

    fn collect_triangles_by_bottom_points(&self, bottom_points: &[Vector3<Float>]) -> Vec<Triangle> {
        let r_factor_vec = Vector3::new(self.radius_factor, self.radius_factor, 1.);
        let top_points: Vec<Vector3<Float>> = bottom_points.iter().map(|p| p.add(self.direction)).collect();
        let mut triangles = self.collect_triangles_by_points(bottom_points, top_points.as_slice());
        if let Some(ref next_branch) = self.next_part {
            triangles.extend(next_branch.collect_triangles_by_bottom_points(top_points.as_slice()));
        }
        triangles
    }
    
    pub fn collect_triangles(&self, point_count: u8) -> Vec<Triangle> {
        debug_assert!(point_count >= 3);
        let bottom_points = self.calculate_base_points(point_count);
        self.collect_triangles_by_bottom_points(bottom_points.as_slice())
      
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
        let triangles = self.root.collect_triangles(16);
        let mesh = Mesh::try_from(&triangles[..])?;
        Ok(mesh)
    }
}
