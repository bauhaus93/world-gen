use std::ops::Add;

use rand;
use rand::Rng;
use rand::prelude::SmallRng;
use rand::SeedableRng;
use glm::{ Vector3, GenNum, normalize };

use crate::utility::Float;
use crate::graphics::mesh::{ Vertex, Triangle, Buffer, Mesh };

pub struct Tree {
    radius: Float,
    length: Float,
    root: Branch
}

struct Branch {
    anchor: Vector3<Float>,
    direction: Vector3<Float>,
    radius_factor_start: Float,
    radius_factor_end: Float,
    length_factor: Float,
    children: Vec<Branch>
}

impl Branch {
    pub fn new<R: Rng + ?Sized>(ancestor_dir: Vector3<Float>, rng: &mut R) -> Self {
        let radius_factor_start = rng.gen_range(0.7, 1.);
        let radius_factor_end = rng.gen_range(0.7 * radius_factor_start, radius_factor_start);
        let mut branch = Self {
            anchor: Vector3::from_s(0.),
            direction: normalize(ancestor_dir.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                               rng.gen_range(-0.1, 0.1),
                                                               0.))),
            radius_factor_start: radius_factor_start,
            radius_factor_end: radius_factor_end,
            length_factor: rng.gen_range(0.1, 1.),
            children: Vec::new()    
        };
        let extensions = rng.gen_range(2, 6);
        info!("Created root branch: anchor = {:?}, dir = {:?}, r = {} -> {}, l = {}, ext = {}", branch.anchor, branch.direction, branch.radius_factor_start, branch.radius_factor_end, branch.length_factor, extensions);
        branch.extend(extensions, rng);
        branch
    }

    fn extend<R: Rng + ?Sized>(&mut self, extension_count: i32, rng: &mut R) {
        let radius_factor_end = if extension_count == 0 {
            0.
        } else {
            rng.gen_range(0.5 * self.radius_factor_end, self.radius_factor_end)
        };
        let mut branch = Self {
            anchor: self.anchor.add(self.direction),
            direction: normalize(self.direction.add(Vector3::new(rng.gen_range(-0.1, 0.1),
                                                                 rng.gen_range(-0.1, 0.1),
                                                                 0.))),
            radius_factor_start: self.radius_factor_end,
            radius_factor_end: radius_factor_end,
            length_factor: rng.gen_range(0.1, 1.),
            children: Vec::new()
        };
        
        info!("Created branch: anchor = {:?}, dir = {:?}, r = {} -> {}, l = {}", branch.anchor, branch.direction, branch.radius_factor_start, branch.radius_factor_end, branch.length_factor);
        if extension_count > 0 {
            branch.extend(extension_count - 1, rng);
        }
        self.children.push(branch);
    }
    
    pub fn collect_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        
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
    
    pub fn collect_triangles(&self) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        
        triangles
    }
}

