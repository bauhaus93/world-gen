use std::ops::Add;

use rand;
use rand::Rng;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use glm::{ Vector3, normalize };

use crate::utility::Float;
use crate::graphics::mesh::{ Vertex, Triangle, Buffer, Mesh };

pub struct Tree {
    radius: Float,
    length: Float,
    root: Branch
}

struct Branch {
    direction: Vector3<Float>,
    radius_factor: Float,
    length_factor: Float,
    children: Vec<Branch>
}

impl Branch {
    pub fn new<R: Rng + ?Sized>(ancestor_dir: Vector3<Float>, rng: &mut R) -> Self {
        let mut branch = Self {
            direction: normalize(ancestor_dir.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                               rng.gen_range(-0.1, 0.1),
                                                               0.))),
            radius_factor: rng.gen_range(0.7, 1.),
            length_factor: rng.gen_range(0.1, 1.),
            children: Vec::new()    
        };
        let extensions = rng.gen_range(2, 6);
        info!("Created root branch: dir = {:?}, r = {}, l = {}, ext = {}", branch.direction, branch.radius_factor, branch.length_factor, extensions);
        branch.extend(extensions, rng);
        branch
    }

    pub fn extend<R: Rng + ?Sized>(&mut self, extension_count: i32, rng: &mut R) {
        let mut branch = Self {
            direction: normalize(self.direction.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                                 rng.gen_range(-0.1, 0.1),
                                                                 0.))),
            radius_factor: self.radius_factor * rng.gen_range(0.8, 1.),
            length_factor: rng.gen_range(0.1, 1.),
            children: Vec::new()
        };
        info!("Created branch: dir = {:?}, r = {}, l = {}", branch.direction, branch.radius_factor, branch.length_factor);
        if extension_count > 0 {
            branch.extend(extension_count - 1, rng);
        }
        self.children.push(branch);
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
    
    pub fn build_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        
        triangles
    }
}

