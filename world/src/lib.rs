#![feature(test)]
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gl;
extern crate glm;
extern crate num_traits;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate byteorder;
extern crate bytes;
extern crate serde;
extern crate serde_yaml;
extern crate rayon;
extern crate test;

extern crate core;

mod chunk;
pub mod erosion;
pub mod height_map;
pub mod noise;
pub mod world;
pub mod world_error;
pub mod architect;
pub mod triangulation;

pub use self::triangulation::triangulate;
pub use self::architect::Architect;
pub use self::height_map::HeightMap;
pub use self::noise::{Noise, NoiseBuilder};
pub use self::world::World;
pub use self::world_error::WorldError;
pub use self::chunk::CHUNK_SIZE;
