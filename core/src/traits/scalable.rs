use crate::Point3f;

pub trait Scalable {
    fn set_scale(&mut self, new_scale: Point3f);
    fn get_scale(&self) -> Point3f;
    fn mod_scale(&mut self, offset: Point3f) {
        self.set_scale(self.get_scale() + offset);
    }
}
