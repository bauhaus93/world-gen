use crate::Point3f;

pub trait Rotatable {
    fn set_rotation(&mut self, new_rotation: Point3f);
    fn get_rotation(&self) -> Point3f;
    fn mod_rotation(&mut self, offset: Point3f) {
        self.set_rotation(self.get_rotation() + offset);
    }
}
