use std::cmp::{ min, max };
use rand::{ Rng, SeedableRng };
use rand::rngs::SmallRng;

use crate::utility::Float;
use super::height_map::HeightMap;

// http://www-ljk.imag.fr/Publications/Basilic/com.lmc.publi.PUBLI_Inproceedings@117681e94b6_fff75c/FastErosion_PG07.pdf

const GRAVITY: Float = 1.;
const SEDIMENT_CAPACITY_CONSTANT: Float = 1.;

pub struct HydraulicErosion {
    height_map: HeightMap,
    rng: SmallRng,
    size: [usize; 2],
    water_height: Vec<Float>,
    flux: Vec<[Float; 4]>,
    velocity: Vec<[Float; 2]>,
    transport_capacacity: Vec<Float>
}

impl HydraulicErosion {
    pub fn new<R: Rng + ?Sized>(height_map: HeightMap, rng_input: &mut R) -> Self {
        let size = self.height_map.get_size();
        let mut water_height =  Vec::new();
        water_height.resize(size[0] * size[1], 0.);
        let mut flux = Vec::new();
        flux.resize(size[0] * size[1], 0.);
        let mut velocity = Vec::new();
        velocity.resize(size[0] * size[1], 0.);
        let mut transport_capacacity = Vec::new();
        transport_capacacity.resize(size[0] * size[1], 0.);
        Self {
            height_map: height_map,
            rng: SmallRng::from_rng(rng_input).unwrap(),
            size: size
            water_height: water_height,
            flux: flux,
            velocity: velocity,
            transport_capacacity: transport_capacacity
        }
    }

    pub fn erode(&mut self) {
        self.rain(10);

        for _i in 0..30 {
            self.update_flux();
            self.apply_flow();
            self.update_transport_capacity();
        }
    }

    fn rain(&mut self, drop_count: u32) {
        for _i in drop_count {
            let drop_index = self.rng.gen_range(0, size[0] * size[1]);
            self.water_height[drop_index] += 1.
        }
    }

    fn update_flux(&mut self) {

        for cell_index in 0..self.flux.len() {
            let mut new_flux: [Float; 4] = [0., 0., 0., 0.];
            let flux_sum = 0.;
            for dir in 0..4 {
                let water_delta= self.get_water_delta(cell_index, dir);
                new_flux[dir] = max(0., self.flux[cell_index][dir] + GRAVITY * water_delta);
                flux_sum += new_flux[dir];
            }
            let k = min(1., self.water_height[cell_index] / flux_sum);
            for dir in 0..4 {
                self.flux[cell_index][dir] = k * new_flux[dir];
            }
        }
    }

    fn apply_flow(&mut self) {
        for cell_index in 0..self.flux.len() {
            let mut water_delta = 0.;
            let mut new_velocity: [Float; 2] = [0., 0.];
            for dir in 0..4 {
                if let Some(nb_index) = self.get_neighbour(cell_index, dir) {
                    let opp_dir = get_opposite_neighbour(dir);
                    let flow_delta = self.flux[nb_index][opp_dir] - self.flux[cell_index][dir]; // inflow - outflow with certain neighbour
                    water_delta += flow_delta;
                    new_velocity[(dir + 1) % 2] += flow_delta;
                }
            }
            self.water_height[cell_index] += water_delta;
            self.velocity[cell_index] = [new_velocity[0] / 2.,
                                         new_velocity[1] / 2.];
        }
    }

    fn update_transport_capacity(&mut self) {
        for cell_index in 0..self.flux.len() {
            self.transport_capacacity[cell_index] =
                SEDIMENT_CAPACITY_CONSTANT *
                self.get_cell_tilt_sinus(cell_index) *
                (self.velocity[cell_index][0].powf(2.) + self.velocity[cell_index][1].powf(2.)).powf(0.5);
        }
    }

    fn get_cell_tilt_sinus(&self, index: usize) {
        let delta_x = (self.get_height_delta(1) - self.get_height_delta(3)) / 2.;
        let delta_y = (self.get_height_delta(0) - self.get_height_delta(2)) / 2.;
        let a = delta_x.powf(2.) + delta_y.powf(2.);
        a.powf(0.5) / (1. + a).powf(0.5)
    }

    fn get_water_delta(&self, index: usize, dir: u8) -> Float {
        match self.get_neighbour(index) {
            Some(nb_index) => self.water_height[index] - self.water_height[nb_index],
            None => 0.
        }
    }

    fn get_height_delta(&self, index: usize, dir: u8) -> Float {
        match self.get_neighbour(index) {
            Some(nb_index) => self.height_map.get_by_index(index) - self.height_map.get_by_index(nb_index),
            None => 0.
        }
    }



    fn get_neighbour(&self, index: usize, dir: u8) -> Option<usize> {
        match dir {
            0 if index >= self.size[0] => Some(index - self.size[0]),                   // TOP
            1 if (index + 1) % self.size[0] != 0 => Some(index + 1),                    // RIGHT
            2 if index + self.size[0] < self.flux.len() => Some(index + self.size[0]),  // BOTTOM
            3 if index % self.size[0] != 0 => Some(index - 1),                          // LEFT
            _ => None
        }
    }

    fn calculate_index(&self, pos: &[usize; 2]) -> usize {
        debug_assert!(pos[0] < self.size[0] && pos[1] < self.size[1]);
        pos[0] + self.size[0] * pos[1]
    }
}

fn get_opposite_neighbour(dir: u8) -> u8 {
    (dir + 2) % 4
}
