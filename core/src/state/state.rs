use super::{Input, StateError};

pub trait State {
    fn update(&mut self, input: &Input) -> Result<(), StateError>;
    fn render(&self) -> Result<(), StateError>;
}
