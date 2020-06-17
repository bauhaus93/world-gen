use rand::{rngs::SmallRng, Rng, SeedableRng};

use super::Architect;
use crate::noise::{Noise, OctavedNoise};
use crate::{Terrain, TerrainSet, TerrainType};
use core::Float;

pub struct ArchitectNoiseFinite {
    architect_infinite: ArchitectNoiseInfinite,
    size: [i64; 2],
}

impl Architect for ArchitectNoiseFinite {
    fn get_height(&self, absolute_pos: [Float; 2]) -> f64 {
        let repeat_pos = self.absolute_to_repeating_pos(absolute_pos);
    }

    fn get_terrain(&self, absolute_pos: [Float; 2]) -> &Terrain {}
}

impl ArchitectNoiseFinite {
    pub fn from_rng<R: Rng + ?Sized>(
        rng: &mut R,
        terrain_set: &TerrainSet,
        size: [i64; 2],
    ) -> ArchitectNoiseInfinite {
        Self {
            architect_infinite: ArchitectNoiseInfinite::from_rng(rng, terrain_set),
            size: size,
        }
    }

    fn absolute_to_repeating_pos(&self, absolute_pos: [Float; 2]) -> [Float; 2] {
        let pos = [
            (absolute_pos[0] as i64) % self.size[0],
            (absolute_pos[1] as i64) % self.size[1],
        ];
    }

    fn get_opposite_pos(&self, repeating_pos: [Float; 2]) -> [Float; 2] {}
}
