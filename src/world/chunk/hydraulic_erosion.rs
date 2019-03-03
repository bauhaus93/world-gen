use std::cmp::{ min, max };
use rand::{ Rng, SeedableRng };
use rand::rngs::SmallRng;

use crate::utility::Float;
use super::height_map::HeightMap;

pub struct HydraulicErosion {
    height_map: HeightMap,
    rng: SmallRng,
    size: [usize; 2],
    water_height: Vec<Float>,
    flux: Vec<[Float; 4]>
}

impl HydraulicErosion {
    pub fn new<R: Rng + ?Sized>(height_map: HeightMap, rng_input: &mut R) -> Self {
        let size = self.height_map.get_size();
        let mut water_height =  Vec::new();
        water_height.resize(size[0] * size[1], 0.);
        let mut flux = Vec::new();
        flux.resize(size[0] * size[1], 0.)
        Self {
            height_map: height_map,
            rng: SmallRng::from_rng(rng_input).unwrap(),
            size: size
            water_height: water_height,
            flux: flux
        }
    }

    pub fn erode(&mut self) {
        self.rain(10);
    }


    pub fn update_flux(&mut self) {
        const GRAVITY: Float = 1.;
        for cell in 0..self.flux.len() {
            let mut new_flux: [Float; 4] = [0., 0., 0., 0.];
            let flux_sum = 0.;
            for dir in 0..4 {
                let water_delta= self.get_water_delta(cell, dir);
                new_flux[dir] = max(0., self.flux[cell][dir] + GRAVITY * water_delta);
                flux_sum += new_flux[dir];
            }
            let k = min(1., self.water_height[cell] / flux_sum);
            for dir in 0..4 {
                self.flux[cell][dir] = k * new_flux[dir];
            }
        }
    }

    fn rain(&mut self, drop_count: u32) {
        for _i in drop_count {
            let drop_index = self.rng.gen_range(0, size[0] * size[1]);
            self.water_height[drop_index] += 1.
        }
    }

    fn get_water_delta(&self, index: usize, dir: u8) -> Float {
        match self.get_neighbour(index) {
            Some(nb_index) => self.water_height[index] - self.water_height[nb_index],
            None => 0.
        }
    }

    fn get_neighbour(&self, index: usize, dir: u8) -> Option<usize> {
        match dir {
            0 if index >= self.size[0] => Some(index - self.size[0]),
            1 if (index + 1) % self.size[0] != 0 => Some(index + 1),
            2 if index + self.size[0] < self.flux.len() => Some(index + self.size[0]),
            3 if index % self.size[0] != 0 => Some(index - 1),
            _ => None
        }
    }

    fn calculate_index(&self, pos: &[usize; 2]) -> usize {
        debug_assert!(pos[0] < self.size[0] && pos[1] < self.size[1]);
        pos[0] + self.size[0] * pos[1]
    }
}
