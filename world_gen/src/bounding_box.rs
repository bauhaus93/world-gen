use std::fmt;

use glm::{ Vector3, GenNum };

use utility::Float;

pub struct BoundingBox {
    points: [Vector3<Float>; 8]
}


impl BoundingBox {

    pub fn from_min_max(min: Vector3<Float>, max: Vector3<Float>) -> BoundingBox {
        BoundingBox {
            points: [
                min,
                Vector3::new(min.x, min.y, max.z),
                Vector3::new(min.x, max.y, min.z),
                Vector3::new(min.x, max.y, max.z),
                Vector3::new(max.x, min.y, min.z),
                Vector3::new(max.x, min.y, max.z),
                Vector3::new(max.x, max.y, min.z),
                max
            ]
        }
    }

    pub fn get_points(&self) -> &[Vector3<Float>] {
        &self.points
    }
}

impl Default for BoundingBox {
    fn default() -> BoundingBox {
        BoundingBox {
            points: [
                Vector3::from_s(0.),
                Vector3::new(0., 0., 1.),
                Vector3::new(0., 1., 0.),
                Vector3::new(0., 1., 1.),
                Vector3::new(1., 0., 0.),
                Vector3::new(1., 0., 1.),
                Vector3::new(1., 1., 0.),
                Vector3::new(1., 1., 1.)
            ]
        }
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoundingBox:")?;
        self.points.iter().try_for_each(|p| write!(f, " {}/{}/{}", p.x, p.y, p.z))
    }
}