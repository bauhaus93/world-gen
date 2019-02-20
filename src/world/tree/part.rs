use std::ops::{ Add, Sub };
use num_traits::One;

use glm::{ Vector3, Vector4, Matrix4, normalize, cross, ext::rotate };

use crate::graphics::transformation::create_rotation_matrix;
use crate::utility::Float;

pub struct Part {
    direction: Vector3<Float>,
    radius: Float,
    length: Float
}

impl Part {
    pub fn new(direction: Vector3<Float>, radius: Float, length: Float) -> Self {
        Self {
            direction: direction,
            radius: radius,
            length: length
        }
    }

    pub fn create_ring_template(&self, count: u32) -> Vec<Vector3<Float>> {
        let right = normalize(cross(self.direction, Vector3::new(0., 0., 1.))) * self.radius;
        let p_base = right.extend(1.);
        let one = Matrix4::<Float>::one();
        let mut points = Vec::new();
        for i in 0..count {
            let rot_mat = rotate(&one, ((360. / (count as f32)) * i as f32).to_radians(), self.direction);
            let p = (rot_mat * p_base).truncate(3);
            points.push(p);
        }
        points
    }

    pub fn next_origin(&self, prev_origin: Vector3<Float>) -> Vector3<Float> {
        prev_origin.add(self.direction * self.length)
    }

    pub fn align_ring(&self, origin: Vector3<Float>, ring_template: &[Vector3<Float>]) -> Vec<Vector3<Float>> {
        let rotation_x = match self.direction[1] {
            d if d < 0. => -d.abs().asin(),
            d => d.asin()
        };
        let rotation_y = match self.direction[0] {
            d if d < 0. => -d.abs().asin(),
            d => d.asin()
        };
        let rot_mat = create_rotation_matrix(Vector3::new(rotation_x, rotation_y, 0.)); 
        let mut aligned_ring = Vec::new();
        for point in ring_template {
            let aligned_point = (rot_mat * ((*point * self.radius).extend(1.))).truncate(3)
                .add(origin);
            aligned_ring.push(aligned_point);
        }
        aligned_ring
    }
}
