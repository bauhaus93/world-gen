
use glm::{ Vector3, Vector4, Matrix4 };

use utility::Float;

pub struct BoundingBox {
    points: [Vector4<Float>; 8]
}

impl BoundingBox {

    pub fn new(min: Vector3<Float>, max: Vector3<Float>) -> BoundingBox {
        BoundingBox {
            points: [
                min.extend(1.),
                Vector4::new(min.x, min.y, max.z, 1.),
                Vector4::new(min.x, max.y, min.z, 1.),
                Vector4::new(min.x, max.y, max.z, 1.),
                Vector4::new(max.x, min.y, min.z, 1.),
                Vector4::new(max.x, min.y, max.z, 1.),
                Vector4::new(max.x, max.y, min.z, 1.),
                max.extend(1.)
            ]
        }
    }

    pub fn is_visible(&self, bb_mvp: Matrix4<Float>) -> bool {
        for p in self.points.iter() {
            let clip = bb_mvp * *p;
            if clip.x.abs() <= clip.w &&
               clip.y.abs() <= clip.w &&
               clip.z.abs() <= clip.w {
                return true;
            }
        }
        false
    }
}