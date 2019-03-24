use std::ops::Add;
use glm::Vector3;

use utility::Float;

pub trait Rotatable {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>);
    fn get_rotation(&self) -> Vector3<Float>;
    fn mod_rotation(&mut self, offset: Vector3<Float>) {
        let new_rotation = self.get_rotation().add(offset);
        self.set_rotation(new_rotation);
    }
} 
