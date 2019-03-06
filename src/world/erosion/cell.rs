use std::rc::Weak;
use std::cell::RefCell;
use std::cmp::{ min, max };

use crate::utility::Float;

pub struct Cell {
    terrain_height: Float,
    water_height: Float,
    flux: [Float; 4],
    velocity: [Float; 2],
    transport_capacacity: Float,
    suspended_sediment: Float,
    neighbours: [Option<Weak<RefCell<Cell>>>; 4]
}



impl Cell {
    pub fn set_neighbours(&mut self, neighbours: [Option<Weak<RefCell<Cell>>>; 4]) {
        self.neighbours = neighbours;
    }
    pub fn set_terrain_height(&mut self, new_height: Float) {
        self.terrain_height = new_height;
    }
    pub fn set_water_height(&mut self, new_height: Float) {
        self.water_height = new_height;
    }
    pub fn mod_water_height(&mut self, height_mod: Float) {
        self.water_height += height_mod;
    }
    pub fn set_flux(&mut self, new_flux: [Float; 4]) {
        self.flux = new_flux;
    }
    pub fn get_flux(&self, dir: u8) -> Float {
        debug_assert!(dir < 4);
        self.flux[dir as usize]
    }
    pub fn get_water_height(&self) -> Float {
        self.water_height
    }
    pub fn get_water_delta(&self, neighbour_cell: &Cell) -> Float {
        self.water_height - neighbour_cell.water_height
    }
    pub fn get_terrain_delta(&self, neighbour_cell: &Cell) -> Float {
        self.terrain_height - neighbour_cell.terrain_height
    }

    pub fn update_flux(&mut self, gravity: Float) {
        let mut new_flux: [Float; 4] = [0., 0., 0., 0.];
        let mut flux_sum = 0.;
        for dir in 0..4 {
            if let Some(nb) = &self.neighbours[dir] {
                if let Some(nb) = nb.upgrade() {
                    let water_delta = self.get_water_delta(&nb.borrow());
                    new_flux[dir] = self.flux[dir] + gravity * water_delta;
                    if new_flux[dir] < 0. {
                        new_flux[dir] = 0.;
                    }
                    flux_sum += new_flux[dir];
                } else {
                    error!("Could not upgrade weak ptr @ update_flux");
                    unreachable!();
                }
            }
        }
        let mut k = self.water_height / flux_sum;
        if k < 1. {
            k = 1.;
        }
        for dir in 0..4 {
            self.flux[dir] = new_flux[dir] * k;
        }
    }
    pub fn apply_flow(&mut self) {
        let mut water_delta = 0.;
        let mut new_velocity: [Float; 2] = [0., 0.];
        for dir in 0..4 {
            if let Some(nb) = &self.neighbours[dir] {
                if let Some(nb) = nb.upgrade() {
                    let opp_dir = get_opposite_dir(dir as u8);
                    let flow_delta = nb.borrow().get_flux(opp_dir) - self.get_flux(dir as u8); // inflow - outflow with certain neighbour
                    water_delta += flow_delta;
                    new_velocity[((dir + 1) % 2) as usize] += flow_delta;
                } else {
                    error!("Could not upgrade weak ptr @ apply_flow");
                    unreachable!();
                }
            }
        }
        self.water_height += water_delta;
        self.velocity = [new_velocity[0] / 2.,
                         new_velocity[1] / 2.];
    }

    pub fn update_transport_capacity(&mut self, sediment_capacity: Float) {
        self.transport_capacacity =
            sediment_capacity *
            self.get_tilt_sinus() *
            (self.velocity[0].powf(2.) + self.velocity[1].powf(2.)).powf(0.5);
    }

    fn get_tilt_sinus(&mut self) -> Float {
        let delta = [self.get_neighbour_terrain_delta(0),
                     self.get_neighbour_terrain_delta(1)];

        let a = delta[0].powf(2.) + delta[1].powf(2.);
        a.powf(0.5) / (1. + a).powf(0.5)
    }

    pub fn get_neighbour_terrain_delta(&mut self, axis: u8) -> Float {
        let neighbours = match axis {
            0 => (&self.neighbours[1], &self.neighbours[3]),  // X axis
            1 => (&self.neighbours[0], &self.neighbours[2]),  // Y axis
            _ => unreachable!()
        };
        match neighbours {
            (Some(a), Some(b)) => {
                if let (Some(a), Some(b)) = (a.upgrade(), b.upgrade()) {
                    self.get_terrain_delta(&a.borrow()) - self.get_terrain_delta(&b.borrow())
                } else {
                    error!("Could not upgrade weak ptr @ get_tilt_sinus");
                    unreachable!();
                }
            },
            (Some(a), None) => {
                if let Some(a) = a.upgrade() {
                    self.get_terrain_delta(&a.borrow())
                } else {
                    error!("Could not upgrade weak ptr @ get_tilt_sinus");
                    unreachable!();
                }
            },
            (None, Some(b)) => {
                if let Some(b) = b.upgrade() {
                    -self.get_terrain_delta(&b.borrow())
                } else {
                    error!("Could not upgrade weak ptr @ get_tilt_sinus");
                    unreachable!();
                }
            },
            (None, None) => 0.
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            terrain_height: 0.,
            water_height: 0.,
            flux: [0., 0., 0., 0.],
            velocity: [0., 0.],
            transport_capacacity: 0.,
            suspended_sediment: 0.,
            neighbours: [None, None, None, None]
        }
    }
}

fn get_opposite_dir(dir: u8) -> u8 {
    (dir + 2) % 4
}
