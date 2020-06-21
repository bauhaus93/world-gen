use std::io::{Write, Error};

pub trait Saveable {
    fn save(&self, writer: &mut impl Write) -> Result<(), Error>;
}
