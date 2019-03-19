
use std::fmt;
use glm;
use glm::GenNum;
use num_traits::One;
use glm::{ Vector3, Matrix4 };

use graphics::{ create_transformation_matrix };
use utility::Float;
use crate::world::traits::{ Translatable, Rotatable, Scalable };


pub struct Model {
    position: Vector3<Float>,
    rotation: Vector3<Float>,
    scale: Vector3<Float>,
    matrix: glm::Matrix4<Float>
}

impl Model {
    fn update_matrix(&mut self) {
        self.matrix = create_transformation_matrix(self.position, self.rotation, self.scale);
    }
    pub fn get_matrix(&self) -> Matrix4<Float> {
        self.matrix.clone()
    }
    pub fn get_matrix_ref(&self) -> &Matrix4<Float> {
        &self.matrix
    }
}

impl Default for Model {
    fn default() -> Self {
        let mut model = Self {
            position: Vector3::<Float>::from_s(0.),
            rotation: Vector3::<Float>::from_s(0.),
            scale: Vector3::<Float>::from_s(1.),
            matrix: Matrix4::<Float>::one()
        };
        model.update_matrix();
        model
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos = {:.2}/{:.2}/{:.2}, rot = {:.2}/{:.2}/{:.2}",
            self.position.x, self.position.y, self.position.z,
            self.rotation.x, self.rotation.y, self.rotation.z)
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
