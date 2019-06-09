use std::fmt;
use glm::{ Matrix4, GenMat, GenNum, Vector3, Vector4 };

use utility::Float;
use crate::traits::Translatable;
use crate::Model;

pub struct Frustum {
    plane_normals: [Vector3<Float>; 6]
}

impl Frustum {
    pub fn from_perspective(mat: &Matrix4<Float>) -> Frustum {
        let trans = mat.transpose();
        let near = trans[2][3] / (trans[2][2] - 1.);
        let far = trans[2][3] / (trans[2][2] + 1.);
        let bottom = near * (trans[1][2] - 1.) / trans[1][1];
        let top = near * (trans[1][2] + 1.) / trans[1][1];
        let left = near * (trans[0][2] - 1.) / trans[0][0];
        let right = near * (trans[0][2] + 1.) / trans[0][0];


        Frustum::default()
    }

    pub fn is_visible(&self, target_mvp: Matrix4<Float>) -> bool {
        let p = target_mvp * Vector4::new(0., 0., 0., 1.);
        p.x.abs() < p.w &&
        p.y.abs() < p.w &&
        p.z.abs() < p.w
    }
}

impl Default for Frustum {
    fn default() -> Frustum {
        Frustum {
            plane_normals: [Vector3::from_s(0.), Vector3::from_s(0.), Vector3::from_s(0.),
                            Vector3::from_s(0.), Vector3::from_s(0.), Vector3::from_s(0.)]
        }
    }
}
