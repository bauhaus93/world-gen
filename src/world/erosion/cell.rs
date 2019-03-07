use std::rc::Weak;
use std::cell::RefCell;

use crate::utility::Float;

pub struct Cell {
    terrain_height: Float,
    water_height: Float,
    flux: [Float; 4],
    velocity: [Float; 2],
    transport_capacacity: Float,
    suspended_sediment: Float,
    transported_sediment: Float,
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

    pub fn update_flux(&mut self, gravity: Float) {
        let mut new_flux: [Float; 4] = [0., 0., 0., 0.];
        let mut flux_sum = 0.;
        for dir in 0..4 {
            if let Some(nb) = &self.neighbours[dir] {
                if let Some(nb) = nb.upgrade() {
                    let terrain_delta = self.get_terrain_delta(&nb.borrow());
                    new_flux[dir] = Float::max(0., self.flux[dir] + gravity * terrain_delta);
                    flux_sum += new_flux[dir];
                } else {
                    error!("Could not upgrade weak ptr @ update_flux");
                    unreachable!();
                }
            }
        }
        let mut k = Float::min(1., self.water_height / flux_sum);
        for dir in 0..4 {
            self.flux[dir] = new_flux[dir] * k;
        }
    }
    pub fn apply_flux(&mut self) {
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

    pub fn apply_erosion_deposition(&mut self, dissolving_factor: Float, deposition_factor: Float) {
        if self.transport_capacacity > self.suspended_sediment {
            let delta = dissolving_factor * (self.transport_capacacity - self.suspended_sediment);
            self.terrain_height -= delta;
            self.suspended_sediment += delta;
        } else {
            let delta = deposition_factor * (self.suspended_sediment - self.transport_capacacity);
            self.terrain_height += delta;
            self.suspended_sediment -= delta;
        }
    }

    pub fn update_transported_sediment(&mut self, time_delta: Float) {
        let prev_offset = [-self.velocity[0] * time_delta,
                           -self.velocity[1] * time_delta];
        self.transported_sediment = self.retrieve_transported_sediment(prev_offset);
    }

    pub fn apply_transported_sediment(&mut self) {
        self.suspended_sediment = self.transported_sediment;
    }

    fn retrieve_transported_sediment(&self, mut offset: [Float; 2]) -> Float {
        if offset[0] >= 0. && offset[1] >= 0. && offset[0] < 1. && offset[1] < 1. {
            let neighbour_sediments = self.get_neighbour_sediments();
            return interpolate(offset, neighbour_sediments);
        }
        for axis in 0..2 {
            if offset[axis] >= 1. {
                if let Some(nb_weak) = &self.neighbours[1 + axis] {  // 1/2 right/bottom
                    if let Some(nb) = nb_weak.upgrade() {
                        offset[axis] -= 1.;
                        return nb.borrow().retrieve_transported_sediment(offset);
                    } else {
                        error!("Could not upgrade weak ptr @ retrieve_transported_sediment");
                        unreachable!();
                    }
                } else {    // point is out of upper grid boundary
                    offset[axis] = 0.;  // shift point to current point for axis
                    return self.retrieve_transported_sediment(offset);  // try again with fixed point
                }
            } else if offset[axis] < 0. {
                if let Some(nb_weak) = &self.neighbours[3 * axis] {  // 0/3 top/left
                    if let Some(nb) = nb_weak.upgrade() {
                        offset[axis] += 1.;
                        return nb.borrow().retrieve_transported_sediment(offset);
                    } else {
                        error!("Could not upgrade weak ptr @ retrieve_transported_sediment");
                        unreachable!();
                    }
                } else {    // point is out of lower grid boundary
                    offset[axis] = 0.;
                    return self.retrieve_transported_sediment(offset);
                }
            }
        }
        unreachable!();
    }

    fn set_flux(&mut self, new_flux: [Float; 4]) {
        self.flux = new_flux;
    }
    fn get_flux(&self, dir: u8) -> Float {
        debug_assert!(dir < 4);
        self.flux[dir as usize]
    }
    fn get_water_height(&self) -> Float {
        self.water_height
    }
    fn get_water_delta(&self, neighbour_cell: &Cell) -> Float {
        self.water_height - neighbour_cell.water_height
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
            if let Some(nb_weak) = &self.neighbours[dir] {
                if let Some(nb) = nb_weak.upgrade() {
                    sediments[dir] = nb.borrow().get_sediment();
                } else {
                    error!("Could not upgrade weak ptr @ get_neighbour_sediments");
                    unreachable!();
                }
            }
        }
        sediments
    }
    fn get_neighbour_terrain_delta(&mut self, axis: u8) -> Float {
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
            transported_sediment: 0.,
            neighbours: [None, None, None, None]
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