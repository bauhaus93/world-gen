use std::ops::Add;
use glm::Vector3;

use utility::Float;

pub trait Translatable {
    fn set_translation(&mut self, new_translation: Vector3<Float>);
    fn get_translation(&self) -> Vector3<Float>;
    fn mod_translation(&mut self, offset: Vector3<Float>) {
        let new_translation = self.get_translation().add(offset);
        self.set_translation(new_translation);
    }
}
