use std::fmt;
use std::ops::{ Add, Sub, Div };

use glm::{ Vector2, Vector3, Vector4, Matrix4, GenNum };

use utility::Float;

pub struct BoundingBox<T>
where T: glm::Primitive {
    center: Vector3<T>,
    size: T
}

#[derive(Eq, PartialEq)]
pub enum Visibility {
    Inside,
    Outside,
    Intersection
}

impl<T> BoundingBox<T>
where T: glm::Primitive
       + Sub<Output = T>
       + Add<Output = T>
       + Div<Output = T>
       + From<i32>
       + Into<f64>
       ,
     Vector3<T>: Sub<Output = Vector3<T>>
               + Add<Output = Vector3<T>>
               + Div<T, Output = Vector3<T>> {

    pub fn new(center: Vector3<T>, size: T) -> Self {
         Self {
             center: center,
             size: size
         }
    }

    pub fn from_min_max(min: Vector3<T>, max: Vector3<T>) -> Self {
        let size = max - min;
        let center = min + size / 2.into();
        let cube_size = match (size.x, size.y, size.z) {
            (x, y, z) if x > y && x > z => x,
            (_, y, z) if y > z => y,
            (_, _, z) => z
        };
        Self {
            center: center,
            size: cube_size
        }
    }

    pub fn get_center(&self) -> Vector3<T> {
        self.center
    }

    pub fn get_center_xy(&self) -> Vector2<T> {
        self.center.truncate(2)
    }

    pub fn get_size(&self) -> T {
        self.size
    }

    pub fn check_visibility(&self, mvp: Matrix4<Float>) -> Visibility {
        self.check_visibility_scaled(mvp, 1.)
    }

    pub fn check_visibility_scaled(&self, mvp: Matrix4<Float>, scale: f32) -> Visibility {
        const FACTORS: [[f64; 3]; 8] = [
            [0.5, 0.5, 0.5],
            [0.5, 0.5, -0.5],
            [0.5, -0.5, 0.5],
            [0.5, -0.5, -0.5],
            [-0.5, 0.5, 0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [-0.5, -0.5, -0.5]
        ];
        let clip_points: Vec<Vector4<Float>> = FACTORS.iter().map(|fact| {
            mvp * Vector4::new((self.center.x.into() + fact[0] * self.size.into()) as f32 * scale,
                               (self.center.y.into() + fact[1] * self.size.into()) as f32 * scale,
                               (self.center.z.into() + fact[2] * self.size.into()) as f32 * scale,
                               1.)
            }).collect();
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
                    return Visibility::Intersection;
                }
            }
            if p_in == 0 {
                return Visibility::Outside;
            }
        }
        Visibility::Inside
    }

}

impl<T> Default for BoundingBox<T>
where T: glm::BaseNum {
    fn default() -> BoundingBox<T> {
        BoundingBox {
            center: Vector3::from_s(T::zero()),
            size: T::one()
        }
    }
}

impl<T> fmt::Display for BoundingBox<T>
where T: fmt::Display + glm::Primitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "center = {}/{}/{}, size {}",
            self.center.x,
            self.center.y,
            self.center.z,
            self.size)
    }
}