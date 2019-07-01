use std::collections::BTreeMap;
use std::cmp::Ordering;

use super::TerrainType;

pub type TerrainSet = BTreeMap<TerrainType, Terrain>;

#[derive(Clone, Copy)]
pub struct Terrain {
    terrain_type: TerrainType,
    surface_texture_layer: u32,
}


impl Terrain {
    pub fn new(terrain_type: TerrainType, surface_texture_layer: u32) -> Terrain {
        Terrain {
            terrain_type: terrain_type,
            surface_texture_layer: surface_texture_layer
        }
    }

    #[allow(unused)]
    pub fn get_type(&self) -> TerrainType {
        self.terrain_type
    }

    pub fn get_layer(&self) -> u32 {
        self.surface_texture_layer
    }
}

impl Ord for Terrain {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.terrain_type.cmp(&rhs.terrain_type)
    }
}

impl PartialOrd for Terrain {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.terrain_type.cmp(&rhs.terrain_type))
    }
}

impl PartialEq for Terrain {
    fn eq(&self, rhs: &Self) -> bool {
        self.terrain_type == rhs.terrain_type
    }
}

impl Eq for Terrain {}