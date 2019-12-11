use crate::update_error::UpdateError;

pub trait Updatable {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError>;
}
