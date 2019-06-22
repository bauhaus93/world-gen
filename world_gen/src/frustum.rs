use std::fmt;

use glm::{ Vector3, GenNum, cross, dot, normalize, length };

use utility::Float;
use crate::BoundingBox;

pub struct Frustum {
    planes: [Plane; 6]
}

struct Plane {
    origin: Vector3<Float>,
    normal: Vector3<Float>
}

impl Plane {
    pub fn new(origin: Vector3<Float>, normal: Vector3<Float>) -> Plane {
        Plane {
            origin: origin,
            normal: normal
        }
    }

    pub fn is_outside(&self, bounding_box: &BoundingBox) -> bool {
        let mut inside = 0;
        let mut outside = 0;
        for p in bounding_box.get_points().iter() {
            if self.distance(*p) < 0. {
                outside += 1;
            } else {
                inside += 1;
            }
            if inside > 0 && outside > 0 {
                return false;
            }
        }
        inside == 0
    }
    
    fn distance(&self, point: Vector3<Float>) -> Float {
        let sn = -dot(self.normal, point - self.origin);
        let sd = dot(self.normal, self.normal);
        let sb = sn / sd;
        let base_point = point + self.normal * sb;
        length(point - base_point)
    }
}

impl Default for Plane {
    fn default() -> Plane {
        Plane {
            origin: Vector3::from_s(0.),
            normal: Vector3::new(0., 0., 1.)
        }
    }
}

impl fmt::Display for Plane {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plane(origin = {}/{}/{}, normal = {}/{}/{})",
            self.origin.x, self.origin.y, self.origin.z,
            self.normal.x, self.normal.y, self.normal.z)
    }
}

impl Frustum {

    pub fn new(origin: Vector3<Float>, orientation: Vector3<Float>, fov: Float, aspect_ratio: Float, near: Float, far: Float) -> Frustum {
        let up = Vector3::new(0., 0., 1.);
        let right = cross(orientation, up);
        let near_h = 2. * Float::tan(fov / 2.) * near;
        let near_w= near_h * aspect_ratio;

        let near_center = origin + orientation * near;
        let far_center = origin +  orientation * far;

        let plane_near = Plane::new(near_center, orientation);
        let plane_far = Plane::new(far_center, -orientation);

        let normal_right = cross(normalize((near_center + right * near_w / 2.) - origin), up);
        let plane_right = Plane::new(origin, normal_right);

        let normal_left = cross(normalize((near_center - right * near_w / 2.) - origin), up);
        let plane_left = Plane::new(origin, normal_left);

        let normal_top = cross(normalize((near_center + up * near_h / 2.) - origin), right);
        let plane_top = Plane::new(origin, normal_top);

        let normal_bot = cross(normalize((near_center - up * near_h / 2.) - origin), right);
        let plane_bot = Plane::new(origin, normal_bot);

        Frustum {
            planes: [
                plane_near, plane_far,
                plane_left, plane_right,
                plane_top, plane_bot
            ]
        }
    }

    pub fn is_visible(&self, bounding_box: &BoundingBox) -> bool {
        for plane in self.planes.iter() {
            if plane.is_outside(bounding_box) {
                return false;
            }
        }
        true
    }
}

impl Default for Frustum {
    fn default() -> Frustum {
        Frustum {
            planes: [Plane::default(),
                     Plane::default(),
                     Plane::default(),
                     Plane::default(),
                     Plane::default(),
                     Plane::default()]
        }
    }
}

impl fmt::Display for Frustum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Frustum:")?;
        self.planes.iter().try_for_each(|p|
            write!(f, "\n{}", p))
    }
}
