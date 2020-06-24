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

extern crate core;

pub mod architect;
mod chunk;
pub mod height_map;
pub mod noise;
mod surface;
pub mod world;
pub mod world_error;
pub mod erosion;

pub use self::height_map::HeightMap;
pub use self::noise::{Noise, NoiseBuilder};
pub use self::world::World;
pub use self::world_error::WorldError;

use self::surface::{Terrain, TerrainSet, TerrainType};
