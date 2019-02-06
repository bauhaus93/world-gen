
pub trait Updatable {
    fn tick(&mut self, time_passed: u32);
}
