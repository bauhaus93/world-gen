use std::fmt;

use glm::{GenNum, Matrix4, Vector3, Vector4};

use crate::{Float, Point3f};

pub struct BoundingBox {
    points: [Vector3<Float>; 8],
}

impl BoundingBox {
    pub fn from_min_max(min: Point3f, max: Point3f) -> BoundingBox {
        BoundingBox {
            points: [
                min.as_glm(),
                Vector3::new(min[0] as Float, min[1] as Float, max[2] as Float),
                Vector3::new(min[0] as Float, max[1] as Float, min[2] as Float),
                Vector3::new(min[0] as Float, max[1] as Float, max[2] as Float),
                Vector3::new(max[0] as Float, min[1] as Float, min[2] as Float),
                Vector3::new(max[0] as Float, min[1] as Float, max[2] as Float),
                Vector3::new(max[0] as Float, max[1] as Float, min[2] as Float),
                max.as_glm(),
            ],
        }
    }

    pub fn is_visible(&self, mvp: Matrix4<Float>) -> bool {
        let clip_points: Vec<Vector4<Float>> =
            self.points.iter().map(|p| mvp * p.extend(1.)).collect();
        for plane in 0..6 {
            let mut p_in = 0;
            let mut p_out = 0;
            for p in clip_points.iter() {
                if plane < 3 {
                    if p[plane] <= p.w {
                        p_in += 1;
                    } else {
                        p_out += 1;
                    }
                } else {
                    if p[plane % 3] >= -p.w {
                        p_in += 1;
                    } else {
                        p_out += 1;
                    }
                }

                if p_in > 0 && p_out > 0 {
                    return true;
                }
            }
            if p_in == 0 {
                return false;
            }
        }
        true
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
                Vector3::new(1., 1., 1.),
            ],
        }
    }
}

impl fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BoundingBox:")?;
        self.points
            .iter()
            .try_for_each(|p| write!(f, " {}/{}/{}", p.x, p.y, p.z))
    }
}
