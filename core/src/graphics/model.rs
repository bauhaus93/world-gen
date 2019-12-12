use glm;
use glm::GenNum;
use glm::{Matrix4, Vector3};
use num_traits::One;
use std::fmt;

use super::create_transformation_matrix;
use crate::traits::{Rotatable, Scalable, Translatable};
use crate::Float;

pub struct Model {
    position: Vector3<Float>,
    rotation: Vector3<Float>,
    scale: Vector3<Float>,
    matrix: Matrix4<Float>,
}

impl Model {
    pub fn get_matrix(&self) -> Matrix4<Float> {
        self.matrix.clone()
    }
    pub fn get_matrix_ref(&self) -> &Matrix4<Float> {
        &self.matrix
    }
    fn update_matrix(&mut self) {
        self.matrix = create_transformation_matrix(self.position, self.rotation, self.scale);
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Self {
            position: Vector3::from_s(0.),
            rotation: Vector3::from_s(0.),
            scale: Vector3::from_s(1.),
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
            self.position.x,
            self.position.y,
            self.position.z,
            self.rotation.x,
            self.rotation.y,
            self.rotation.z
        )
    }
}

impl Translatable for Model {
    fn set_translation(&mut self, new_translation: Vector3<Float>) {
        self.position = new_translation;
        self.update_matrix();
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.position.clone()
    }
}

impl Rotatable for Model {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>) {
        const DOUBLE_PI: Float = std::f32::consts::PI as Float * 2.;
        self.rotation = new_rotation;
        for value in self.rotation.as_array_mut().iter_mut() {
            if *value >= DOUBLE_PI {
                *value -= DOUBLE_PI;
            } else if *value < 0. {
                *value += DOUBLE_PI;
            }
        }
        self.update_matrix();
    }
    fn get_rotation(&self) -> Vector3<Float> {
        self.rotation.clone()
    }
}

impl Scalable for Model {
    fn set_scale(&mut self, new_scale: Vector3<Float>) {
        const MIN_SCALE: Float = 1e-3;
        self.scale = new_scale;
        for value in self.scale.as_array_mut().iter_mut() {
            if *value < MIN_SCALE {
                *value = MIN_SCALE;
            }
        }
        self.update_matrix();
    }
    fn get_scale(&self) -> Vector3<Float> {
        self.scale.clone()
    }
}
