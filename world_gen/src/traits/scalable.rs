use std::ops::Add;
use glm::Vector3;

use utility::Float;

pub trait Scalable {
    fn set_scale(&mut self, new_scale: Vector3<Float>);
    fn get_scale(&self) -> Vector3<Float>;
    fn mod_scale(&mut self, offset: Vector3<Float>) {
        let new_scale = self.get_scale().add(offset);
        self.set_scale(new_scale);
    }
}
