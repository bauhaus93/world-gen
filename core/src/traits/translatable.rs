use crate::Point3f;

pub trait Translatable {
    fn set_translation(&mut self, new_translation: Point3f);
    fn get_translation(&self) -> Point3f;
    fn mod_translation(&mut self, offset: Point3f) {
        self.set_translation(self.get_translation() + offset);
    }
}
