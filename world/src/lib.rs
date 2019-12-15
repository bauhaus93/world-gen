#[macro_use]
extern crate log;
extern crate env_logger;
extern crate gl;
extern crate glm;
extern crate num_traits;
extern crate rand;
#[macro_use]
extern crate lazy_static;
extern crate serde;
extern crate serde_yaml;

extern crate core;

pub mod world;
pub mod world_error;
pub mod chunk;
pub mod erosion;
mod noise;
mod surface;

pub use self::world::World;
pub use self::world_error::WorldError;
pub use self::chunk::HeightMap;
pub use self::erosion::HydraulicErosion;

use self::surface::{ Terrain, TerrainSet, TerrainType };




