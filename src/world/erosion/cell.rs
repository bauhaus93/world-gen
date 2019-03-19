use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use std::ptr;
use std::f32::{ NAN };

use crate::utility::Float;

pub struct Cell {
    terrain_height: Float,
    water_height: Float,
    flux: [Float; 4],
    velocity: [Float; 2],
    transport_capacacity: Float,
    suspended_sediment: Float,
    transported_sediment: Float,
    neighbours: [*const Cell; 4]
}

impl Cell {
    pub fn set_neighbours(&mut self, neighbours: [*const Cell; 4]) {
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
    pub fn get_terrain_height(&self) -> Float {
        self.terrain_height
    }

    pub fn update_flux(&mut self, gravity: Float, time_delta: Float) {
        let mut new_flux: [Float; 4] = [0., 0., 0., 0.];
        let mut flux_sum = 0.;
        for dir in 0..4 {
            unsafe {
                if let Some(nb) = self.neighbours[dir].as_ref() {
                    let water_level_delta = self.get_water_level_delta(nb);
                    new_flux[dir] = Float::max(0., self.flux[dir] + 0.0005 * time_delta * gravity * water_level_delta);
                    flux_sum += new_flux[dir];
                }
            }
        }
        if flux_sum > 0. {
            let mut k = Float::min(1., self.water_height / (flux_sum * time_delta));
            for dir in 0..4 {
                self.flux[dir] = new_flux[dir] * k;
            }
        }
    }
    pub fn apply_flux(&mut self, time_delta: Float) {
        let mut water_delta = 0.;
        let mut total_flow_delta: [Float; 2] = [0., 0.];
        for dir in 0..4 {
            unsafe {
                if let Some(nb) = self.neighbours[dir].as_ref() {
                    let opp_dir = get_opposite_dir(dir as u8);
                    let flow_delta = nb.get_flux(opp_dir) - self.get_flux(dir as u8); // inflow - outflow with certain neighbour
                    water_delta += flow_delta;
                    total_flow_delta[((dir + 1) % 2) as usize] += flow_delta;
                }
            }
        }
        self.water_height += water_delta * time_delta;
        if self.water_height < 0. {
            self.water_height = 0.;
        }
        if water_delta > 0. {
            for axis in 0..2 {
                self.velocity[axis] = total_flow_delta[axis] / 2.;
            }
        } else {
            for axis in 0..2 {
                self.velocity[axis] = 0.;
            }
        }
        debug_assert!(!self.velocity[0].is_nan() && !self.velocity[1].is_nan());
    }

    pub fn update_transport_capacity(&mut self, sediment_capacity: Float) {
        self.transport_capacacity =
            sediment_capacity *
            self.get_tilt_sinus() *
            (self.velocity[0].powf(2.) + self.velocity[1].powf(2.)).powf(0.5);
    }

    pub fn apply_erosion_deposition(&mut self, time_delta: Float, dissolving_factor: Float, deposition_factor: Float) {
        if self.transport_capacacity > self.suspended_sediment {
            let mut delta = dissolving_factor * (self.transport_capacacity - self.suspended_sediment);
            /*let source_height = self.calculate_source_height(time_delta);
            if delta > source_height - self.terrain_height {
                delta = source_height - self.terrain_height;
                if delta < 0. {
                    delta = 0.;
                }
            }*/
            /*if delta > 1e-3 {
                info!("water_level = {}, suspended_sediment = {}, transport_capacity = {}, delta = {}", self.water_height, self.suspended_sediment, self.transport_capacacity, delta);
            }*/
            self.terrain_height -= delta;
            self.suspended_sediment += delta;
        } else {
            let mut delta = deposition_factor * (self.suspended_sediment - self.transport_capacacity);
            let source_height = self.calculate_source_height(time_delta);
            if delta > source_height - self.terrain_height {
                delta = source_height - self.terrain_height;
                if delta < 0. {
                    delta = 0.;
                }
            }
            self.terrain_height += delta;
            self.suspended_sediment -= delta;
        }
    }

    pub fn update_transported_sediment(&mut self, time_delta: Float) {
        self.transported_sediment = self.calculate_transported_sediment(time_delta);
    }

    pub fn apply_transported_sediment(&mut self) {
        self.suspended_sediment = self.transported_sediment;
    }

    pub fn apply_evaporation(&mut self, evaporation_factor: Float, time_delta: Float) {
        debug_assert!(evaporation_factor * time_delta <= 1.);
        self.water_height *= (1. - evaporation_factor * time_delta);
    }

    pub fn get_neighbour(&self, dir: u8) -> Option<*const Cell> {
        if !self.neighbours[dir as usize].is_null() {
            Some(self.neighbours[dir as usize])
        } else {
            None
        }
    }

    fn interpolate_height(&self, mut offset: [Float; 2]) -> Float {
        if let Some(cell) = self.get_cell(offset) {
            let neighbour_heights = cell.get_neighbour_heights();
            interpolate(offset, neighbour_heights)
        } else {
            info!("offset = {:?}", offset);
            unreachable!();
        }
    }

    fn calculate_transported_sediment(&self, time_delta: Float) -> Float {
        let prev_offset = [-self.velocity[0] * time_delta,
                           -self.velocity[1] * time_delta];
        self.retrieve_transported_sediment(prev_offset)
    }

    fn calculate_source_height(&self, time_delta: Float) -> Float {
        let prev_offset = [-self.velocity[0] * time_delta,
                           -self.velocity[1] * time_delta];
        self.interpolate_height(prev_offset)
    }

    fn retrieve_transported_sediment(&self, mut offset: [Float; 2]) -> Float {
        if let Some(cell) = self.get_cell(offset) {
            let neighbour_sediments = cell.get_neighbour_sediments();
            interpolate(offset, neighbour_sediments)
        } else {
            info!("offset = {:?}", offset);
            unreachable!();
        }
    }

    fn get_cell(&self, mut offset: [Float; 2]) -> Option<&Cell> {
        if offset[0] >= 0. && offset[1] >= 0. && offset[0] < 1. && offset[1] < 1. {
            return Some(&self)
        }
        for axis in 0..2 {
            if offset[axis] >= 1. {
                unsafe {
                    if let Some(nb) = self.neighbours[1 + axis].as_ref() {  // 1/2 right/bottom
                        offset[axis] -= 1.;
                        return nb.get_cell(offset);
                    } else {    // point is out of upper grid boundary
                        offset[axis] = 0.;  // shift point to current point for axis
                        return self.get_cell(offset);  // try again with fixed point
                    }
                }
            } else if offset[axis] < 0. {
                unsafe {
                    if let Some(nb) = self.neighbours[3 * axis].as_ref() {  // 0/3 top/left
                        offset[axis] += 1.;
                        return nb.get_cell(offset);
                    } else {    // point is out of lower grid boundary
                        offset[axis] = 0.;
                        return self.get_cell(offset);
                    }
                }
            }
        }
        None
    }

    fn get_flux(&self, dir: u8) -> Float {
        debug_assert!(dir < 4);
        self.flux[dir as usize]
    }
    fn get_water_height(&self) -> Float {
        self.water_height
    }
    fn get_water_level_delta(&self, neighbour_cell: &Cell) -> Float {
        self.terrain_height + self.water_height - (neighbour_cell.terrain_height + neighbour_cell.water_height)
    }
    fn get_terrain_delta(&self, neighbour_cell: &Cell) -> Float {
        self.terrain_height - neighbour_cell.terrain_height
    }
    fn get_sediment(&self) -> Float {
        self.suspended_sediment
    }
    fn get_tilt_sinus(&mut self) -> Float {
        let delta = [self.get_neighbour_terrain_delta(0),
                     self.get_neighbour_terrain_delta(1)];
        let a = delta[0].powf(2.) + delta[1].powf(2.);
        a.powf(0.5) / (1. + a).powf(0.5)
    }
    fn get_neighbour_sediments(&self) -> [Float; 4] {
        let mut sediments = [0., 0., 0., 0.];
        for dir in 0..4 {
            unsafe {
                if let Some(nb) = self.neighbours[dir].as_ref() {
                    sediments[dir] = nb.get_sediment();
                }
            }
        }
        sediments
    }
    fn get_neighbour_heights(&self) -> [Float; 4] {
        let mut heights = [NAN, NAN, NAN, NAN];
        let mut sum = 0.;
        let mut c = 0;
        for dir in 0..4 {
            unsafe {
                if let Some(nb) = self.neighbours[dir].as_ref() {
                    heights[dir] = nb.get_terrain_height();
                    sum += heights[dir];
                    c += 1;
                }
            }
        }
        let avg = sum / c as Float;
        for dir in 0..4 {
            if heights[dir].is_nan() {
                heights[dir] = avg;
            }
        }
        heights  
    }
    fn get_neighbour_terrain_delta(&mut self, axis: u8) -> Float {
        let neighbours = unsafe {
            match axis {
                0 => (self.neighbours[1].as_ref(), self.neighbours[3].as_ref()),  // X axis
                1 => (self.neighbours[0].as_ref(), self.neighbours[2].as_ref()),  // Y axis
                _ => unreachable!()
            }
        };
        match neighbours {
            (Some(a), Some(b)) => {
                self.get_terrain_delta(a) - self.get_terrain_delta(b)
            },
            (Some(a), None) => {
                self.get_terrain_delta(a)
            },
            (None, Some(b)) => {
                -self.get_terrain_delta(b)
            },
            (None, None) => 0.
        }
    }
    pub fn check_sanity(&self) {
        if self.water_height < 0. {
            error!("cell sanity check: water_height = {}", self.water_height);
            unreachable!();
        }
        for dir in 0..4 {
            if self.flux[dir] < 0. {
                error!("cell sanity check: flux = {:?}, ", self.flux);
                unreachable!();
            }
        }
        if self.transport_capacacity < 0. {
            error!("cell sanity check: transport_capacity = {}", self.transport_capacacity);
            unreachable!();
        }
        if self.suspended_sediment < 0. {
            error!("cell sanity check: suspended_sediment = {}", self.suspended_sediment);
            unreachable!();
        }
        if self.transported_sediment < 0. {
            error!("cell sanity check: transported_sediment = {}", self.transported_sediment);
            unreachable!();
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
            transported_sediment: 0.,
            neighbours: [ptr::null(), ptr::null(), ptr::null(), ptr::null()]
        }
    }
}

fn get_opposite_dir(dir: u8) -> u8 {
    (dir + 2) % 4
}

fn interpolate(p: [Float; 2], reference: [Float; 4]) -> Float {
    let anchor = [p[0].floor() as i32, p[1].floor() as i32];
    let a = anchor[0] as Float + 1. - p[0];
    let b = p[0] - anchor[0] as Float;
    let r_1 = a * reference[0] + b * reference[1];
    let r_2 = a * reference[2] + b * reference[3];
    let c = anchor[1] as Float + 1. - p[1];
    let d = p[1] - anchor[1] as Float;
    c * r_1 + d * r_2
}