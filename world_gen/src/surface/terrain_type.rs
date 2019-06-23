use std::fmt;

use serde::Deserialize;

#[derive(Deserialize, Ord, Eq, PartialOrd, PartialEq, Clone, Copy)]
pub enum TerrainType {
    Grass,
    Mud,
    Rock
}

impl fmt::Display for TerrainType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TerrainType::Grass => write!(f, "grass"),
            TerrainType::Mud => write!(f, "mud"),
            TerrainType::Rock => write!(f, "rock")
        }
    }
}

