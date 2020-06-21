use std::io::{Read, Error};

pub trait Loadable {
    fn load(&self, reader: &impl Read) -> Result<(), Error>;
}
