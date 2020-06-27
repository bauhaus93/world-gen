use glm;
use glm::Matrix4;
use num_traits::One;
use std::fmt;

use super::create_transformation_matrix;
use crate::traits::{Rotatable, Scalable, Translatable};
use crate::Point3f;

pub struct Model {
    position: Point3f,
    rotation: Point3f,
    scale: Point3f,
    matrix: Matrix4<f32>,
}

impl Model {

    pub fn get_distance(&self, point: Point3f) -> f32 {
        (self.position - point).length()
    }
    pub fn get_matrix(&self) -> Matrix4<f32> {
        self.matrix.clone()
    }
    pub fn get_matrix_ref(&self) -> &Matrix4<f32> {
        &self.matrix
    }
    fn update_matrix(&mut self) {
        self.matrix = create_transformation_matrix(self.position, self.rotation, self.scale);
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Self {
            position: Point3f::from_scalar(0.),
            rotation: Point3f::from_scalar(0.),
            scale: Point3f::from_scalar(1.),
            matrix: Matrix4::one(),
        };
        model.update_matrix();
        model
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "pos = {:.2}/{:.2}/{:.2}, rot = {:.2}/{:.2}/{:.2}",
            self.position[0],
            self.position[1],
            self.position[2],
            self.rotation[0],
            self.rotation[1],
            self.rotation[2]
        )
    }
}

impl Translatable for Model {
    fn set_translation(&mut self, new_translation: Point3f) {
        self.position = new_translation;
        self.update_matrix();
    }
    fn get_translation(&self) -> Point3f {
        self.position
    }
}

impl Rotatable for Model {
    fn set_rotation(&mut self, new_rotation: Point3f) {
        const DOUBLE_PI: f32 = std::f32::consts::PI * 2.;
        self.rotation = new_rotation;
        for i in 0..3 {
            if self.rotation[i] >= DOUBLE_PI {
                self.rotation[i] -= DOUBLE_PI;
            } else if self.rotation[i] < 0. {
                self.rotation[i] += DOUBLE_PI;
            }
        }
        self.update_matrix();
    }
    fn get_rotation(&self) -> Point3f {
        self.rotation
    }
}

impl Scalable for Model {
    fn set_scale(&mut self, new_scale: Point3f) {
        const MIN_SCALE: f32 = 1e-3;
        self.scale = new_scale;
        self.scale.clamp_min(MIN_SCALE);
        self.update_matrix();
    }
    fn get_scale(&self) -> Point3f {
        self.scale
    }
}
