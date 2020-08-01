use rand::{rngs::SmallRng, Rng};
use rayon::prelude::*;

use crate::HeightMap;
use core::{Point2f, Point2i, Point3f, Seed};

type Flux = [f32; 4];

// TODO: Proper values for constants

const WATER_AMOUNT: f32 = 10.;
const PIPE_AREA: f32 = 0.1;
const PIPE_LENGTH: f32 = 0.1;
const GRID_DISTANCE_X: f32 = 0.1;
const GRID_DISTANCE_Y: f32 = 0.1;
const GRAVITY: f32 = 0.1;
const DELTA_TIME: f32 = 1e-3;
const SEDIMENT_CAPACITY_CONSTANT: f32 = 0.1;
const DISSOLVING_CONSTANT: f32 = 5e-1;
const DEPOSITION_CONSTANT: f32 = 1e-1;
const EVAPORATION_CONSTANT: f32 = 1e-2;
const MIN_TILT: f32 = 0.2;

#[derive(Clone)]
pub struct Model {
    size: usize,
    terrain_height: Vec<f32>,
    water_height: Vec<f32>,
    suspended_sediment: Vec<f32>,
    outflow_flux: Vec<Flux>,
    velocity: Vec<Point2f>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn get_opposite(&self) -> Self {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn get_index(&self) -> usize {
        match *self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }

    pub fn as_slice() -> &'static [Direction] {
        &[
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

impl Model {
    pub fn new_zeroed(size: usize) -> Self {
        let mut m = Self {
            size: size,
            terrain_height: Vec::with_capacity(size * size),
            water_height: Vec::with_capacity(size * size),
            suspended_sediment: Vec::with_capacity(size * size),
            outflow_flux: Vec::with_capacity(size * size),
            velocity: Vec::with_capacity(size * size),
        };
        m.terrain_height.resize(size * size, 0.);
        m.water_height.resize(size * size, 0.);
        m.suspended_sediment.resize(size * size, 0.);
        m.outflow_flux.resize(size * size, [0., 0., 0., 0.]);
        m.velocity.resize(size * size, Point2f::from_scalar(0.));
        m
    }

    pub fn get_total_water(&self) -> f32 {
        self.water_height.iter().fold(0., |acc, h| acc + h)
    }

    pub fn get_total_terrain_height(&self) -> f32 {
        self.terrain_height.iter().fold(0., |acc, h| acc + h)
    }

    pub fn get_total_suspended_sediment(&self) -> f32 {
        self.suspended_sediment.iter().fold(0., |acc, h| acc + h)
    }

    pub fn get_total_velocity(&self) -> f32 {
        self.velocity.iter().fold(0., |acc, v| acc + v.length())
    }

    pub fn get_terrain_suspended_ratio(&self) -> f32 {
        self.get_total_suspended_sediment() / self.get_total_terrain_height()
    }

    pub fn run(mut self, count: usize, seed: Seed) -> Self {
        info!("Starting erosion simulation with {} turns", count);
        let mut rng: SmallRng = seed.into();
        for i in 0..count {
            self = self.run_once(&mut rng);
            if i % usize::max(1, count / 20) == 0 {
                info!(
                    "Progress: {:2}% | terrain height: {:.0} | susp sed: {:.2} | total water: {:.2}",
                    100 * i / count,
                    self.get_total_terrain_height(),
                    self.get_total_suspended_sediment(),
                    self.get_total_water()
                );
            }
        }
        self.finish();
        self
    }

    pub fn finish(&mut self) {
        self.deposit_all_suspended();
    }

    pub fn consume(mut self) -> Vec<f32> {
        self.finish();
        self.terrain_height
    }

    fn run_once(mut self, rng: &mut impl Rng) -> Self {
        self.rain(DELTA_TIME * WATER_AMOUNT, 100, rng);
        //self.river((self.size * (self.size - 1)) / 2, 2.);

        let new_flux = self.calculate_flux();
        self.update_flux(new_flux);
        self.apply_flux_to_waterlevel();

        self.update_velocity();
        self.apply_erosion_deposition();

        let transported_sediment = self.calculate_sediment_transportation();
        self.update_suspended_sediment(transported_sediment);

        self.apply_evaporation();

        self
    }

    pub fn get_water_height(&self, pos: Point2i) -> f32 {
        self.water_height[self.get_index(pos)]
    }
    pub fn get_terrain_height(&self, pos: Point2i) -> f32 {
        self.terrain_height[self.get_index(pos)]
    }
    pub fn get_velocity(&self, pos: Point2i) -> Point2f {
        self.velocity[self.get_index(pos)]
    }
    pub fn get_suspended_sediment(&self, pos: Point2i) -> f32 {
        self.suspended_sediment[self.get_index(pos)]
    }

    fn rain(&mut self, total_amount: f32, drop_count: usize, rng: &mut impl Rng) {
        let drop_size = total_amount / drop_count as f32;
        for _ in 0..drop_count {
            let index: usize = rng.gen_range(0, self.water_height.len());
            self.water_height[index] += drop_size;
        }
    }

    #[allow(dead_code)]
    fn river(&mut self, source_index: usize, total_amount: f32) {
        self.water_height[source_index] += total_amount;
    }

    fn calculate_flux_for_cell(&self, cell: usize) -> Flux {
        let mut new_flux = [0.; 4];

        for dir in Direction::as_slice().iter() {
            let nb = self.get_neighbour_index(cell, *dir);
            let delta_height = self.terrain_height[cell] + self.water_height[cell]
                - self.terrain_height[nb]
                - self.water_height[nb];
            if delta_height.abs() < 1e-6 {
                new_flux[dir.get_index()] = 0.;
            } else {
                new_flux[dir.get_index()] = f32::max(
                    0.,
                    self.outflow_flux[cell][dir.get_index()]
                        + DELTA_TIME * PIPE_AREA * ((GRAVITY * delta_height) / PIPE_LENGTH),
                );
            }
        }
        if new_flux.iter().any(|e| e.abs() > 0.) {
            let scaling = f32::min(
                1.,
                (self.water_height[cell] * GRID_DISTANCE_X * GRID_DISTANCE_Y)
                    / (new_flux.iter().fold(0., |acc, f| acc + f) * DELTA_TIME),
            );
            for dir in Direction::as_slice().iter() {
                new_flux[dir.get_index()] *= scaling;
                //println!("new = {}", new_flux[i][dir.get_index()]);
            }
        }
        new_flux
    }

    fn calculate_flux(&self) -> Vec<Flux> {
        (0..self.size * self.size)
            .into_par_iter()
            .map(|i| self.calculate_flux_for_cell(i))
            .collect()
    }
    fn update_flux(&mut self, new_flux: Vec<Flux>) {
        self.outflow_flux = new_flux;
    }

    fn calculate_flux_water_delta_for_cell(&self, cell: usize) -> f32 {
        let inflow = Direction::as_slice()
            .iter()
            .map(|d| {
                self.outflow_flux[self.get_neighbour_index(cell, *d)][d.get_opposite().get_index()]
            })
            .fold(0., |acc, flux| acc + flux);
        let outflow = Direction::as_slice().iter().fold(0., |acc, dir| {
            acc + self.outflow_flux[cell][dir.get_index()]
        });
        let delta = (DELTA_TIME * (inflow - outflow)) / (GRID_DISTANCE_X * GRID_DISTANCE_Y);
        delta
    }

    fn apply_flux_to_waterlevel(&mut self) {
        let water_delta: Vec<f32> = (0..self.size * self.size)
            .into_par_iter()
            .map(|i| self.calculate_flux_water_delta_for_cell(i))
            .collect();
        for i in 0..self.size * self.size {
            self.water_height[i] += water_delta[i];
        }
    }

    fn calculate_velocity_for_cell(&self, cell: usize) -> Point2f {
        let flux_left = self.outflow_flux[self.get_neighbour_index(cell, Direction::Left)]
            [Direction::Right.get_index()]
            - self.outflow_flux[cell][Direction::Left.get_index()];
        let flux_right = self.outflow_flux[cell][Direction::Right.get_index()]
            - self.outflow_flux[self.get_neighbour_index(cell, Direction::Right)]
                [Direction::Left.get_index()];
        let flux_up = self.outflow_flux[self.get_neighbour_index(cell, Direction::Up)]
            [Direction::Down.get_index()]
            - self.outflow_flux[cell][Direction::Up.get_index()];
        let flux_down = self.outflow_flux[cell][Direction::Down.get_index()]
            - self.outflow_flux[self.get_neighbour_index(cell, Direction::Down)]
                [Direction::Up.get_index()];

        let flux_x = (flux_left + flux_right) / 2.;
        let flux_y = (flux_up + flux_down) / 2.;
        let u = flux_x / f32::max(1e-3, GRID_DISTANCE_Y * self.water_height[cell]); // possible problem: not using average of water height between intermediate steps (instead of (d1+d2)/2, just using d2)
        let v = flux_y / f32::max(1e-3, GRID_DISTANCE_X * self.water_height[cell]); // possible problem: not using average of water height between intermediate steps (instead of (d1+d2)/2, just using d2)
        let velocity = Point2f::new(
            f32::min(u, GRID_DISTANCE_X / DELTA_TIME),
            f32::min(v, GRID_DISTANCE_Y / DELTA_TIME),
        );
        debug_assert!(!velocity[0].is_infinite() && !velocity[0].is_nan());
        debug_assert!(!velocity[1].is_infinite() && !velocity[1].is_nan());
        velocity
    }

    fn update_velocity(&mut self) {
        let new_velocity: Vec<Point2f> = (0..self.size * self.size)
            .into_par_iter()
            .map(|i| self.calculate_velocity_for_cell(i))
            .collect();
        for i in 0..self.size * self.size {
            self.velocity[i] = new_velocity[i];
        }
    }

    fn calculate_suspended_delta_for_cell(&self, cell: usize) -> f32 {
        let transport_capacity = SEDIMENT_CAPACITY_CONSTANT
            * f32::max(MIN_TILT, self.get_tilt_angle(cell)).sin()
            * self.velocity[cell].length();
        if transport_capacity > self.suspended_sediment[cell] {
            let suspended_sediment =
                DISSOLVING_CONSTANT * (transport_capacity - self.suspended_sediment[cell]);
            suspended_sediment
        } else {
            let deposited_sediment =
                DEPOSITION_CONSTANT * (self.suspended_sediment[cell] - transport_capacity);
            -deposited_sediment
        }
    }

    fn apply_erosion_deposition(&mut self) {
        let sediment_delta: Vec<f32> = (0..self.size * self.size)
            .into_par_iter()
            .map(|i| self.calculate_suspended_delta_for_cell(i))
            .collect();
        for i in 0..self.size * self.size {
            self.terrain_height[i] -= sediment_delta[i];
            self.suspended_sediment[i] += sediment_delta[i];
        }
    }

    fn calculate_sediment_transportation_for_cell(&self, cell: usize) -> f32 {
        let mut source_pos = Point2f::new(
            (cell % self.size) as f32 - self.velocity[cell][0] * DELTA_TIME,
            (cell / self.size) as f32 - self.velocity[cell][1] * DELTA_TIME,
        );

        if source_pos[0] < 0. {
            source_pos[0] += self.size as f32;
        }
        if source_pos[0] >= self.size as f32 {
            source_pos[0] -= self.size as f32;
        }
        if source_pos[1] < 0. {
            source_pos[1] += self.size as f32;
        }
        if source_pos[1] >= self.size as f32 {
            source_pos[1] -= self.size as f32;
        }
        assert!(source_pos[0] >= 0. && source_pos[0] < self.size as f32);
        assert!(source_pos[1] >= 0. && source_pos[1] < self.size as f32);

        let grid_ul_index = source_pos[0] as usize + source_pos[1] as usize * self.size;
        let grid_ur_index = self.get_neighbour_index(grid_ul_index, Direction::Right);
        let grid_dl_index = self.get_neighbour_index(grid_ul_index, Direction::Down);
        let grid_dr_index = self.get_neighbour_index(grid_dl_index, Direction::Right);
        // overwrites existing delta from deposition/suspending phase, which should already have been
        // applied to the model
        interpolate(
            source_pos,
            [
                self.suspended_sediment[grid_ul_index],
                self.suspended_sediment[grid_ur_index],
                self.suspended_sediment[grid_dl_index],
                self.suspended_sediment[grid_dr_index],
            ],
        )
    }

    fn calculate_sediment_transportation(&self) -> Vec<f32> {
        (0..self.size * self.size)
            .into_par_iter()
            .map(|i| self.calculate_sediment_transportation_for_cell(i))
            .collect()
    }

    fn update_suspended_sediment(&mut self, new_sediment: Vec<f32>) {
        self.suspended_sediment = new_sediment;
    }

    fn apply_evaporation(&mut self) {
        let evap_factor = f32::max(0., 1. - EVAPORATION_CONSTANT * DELTA_TIME);
        for i in 0..self.size * self.size {
            self.water_height[i] *= evap_factor;
        }
    }

    fn get_index(&self, pos: Point2i) -> usize {
        pos[0] as usize + self.size * (pos[1] as usize)
    }

    fn get_neighbour_index(&self, cell: usize, dir: Direction) -> usize {
        match dir {
            Direction::Up if cell < self.size => self.size * self.size - (self.size - cell),
            Direction::Right if (cell + 1) % self.size == 0 => cell + 1 - self.size,
            Direction::Down if cell + self.size >= self.size * self.size => cell % self.size,
            Direction::Left if cell % self.size == 0 => cell + self.size - 1,
            Direction::Up => cell - self.size,
            Direction::Right => cell + 1,
            Direction::Down => cell + self.size,
            Direction::Left => cell - 1,
        }
    }

    fn get_tilt_angle(&self, cell: usize) -> f32 {
        let h_up = self.terrain_height[self.get_neighbour_index(cell, Direction::Up)];
        let h_down = self.terrain_height[self.get_neighbour_index(cell, Direction::Down)];
        let h_left = self.terrain_height[self.get_neighbour_index(cell, Direction::Left)];
        let h_right = self.terrain_height[self.get_neighbour_index(cell, Direction::Right)];

        let v_ud = Point3f::new(0., -2., h_up - h_down).as_normalized();
        let v_lr = Point3f::new(-2., 0., h_left - h_right).as_normalized();

        let cos_tilt = v_lr.cross(&v_ud).dot(&Point3f::new(0., 0., 1.));
        cos_tilt.acos()
    }

    fn deposit_all_suspended(&mut self) {
        for i in 0..self.size * self.size {
            self.terrain_height[i] += self.suspended_sediment[i];
        }
    }
}

impl From<HeightMap> for Model {
    fn from(hm: HeightMap) -> Self {
        let size = hm.get_size() as usize;
        let mut m = Self {
            size: size,
            terrain_height: Vec::from(hm.get_list()),
            water_height: Vec::with_capacity(size * size),
            suspended_sediment: Vec::with_capacity(size * size),
            outflow_flux: Vec::with_capacity(size * size),
            velocity: Vec::with_capacity(size * size),
        };
        m.water_height.resize(size * size, 0.);
        m.suspended_sediment.resize(size * size, 0.);
        m.outflow_flux.resize(size * size, [0., 0., 0., 0.]);
        m.velocity.resize(size * size, Point2f::from_scalar(0.));
        m
    }
}

impl Into<HeightMap> for Model {
    fn into(self) -> HeightMap {
        HeightMap::from_list(self.size as i32, 1., self.terrain_height.as_slice())
    }
}

fn interpolate(p: Point2f, reference: [f32; 4]) -> f32 {
    let anchor = [p[0].floor() as i32, p[1].floor() as i32];
    let a = anchor[0] as f32 + 1. - p[0];
    let b = p[0] - anchor[0] as f32;
    let r_1 = a * reference[0] + b * reference[1];
    let r_2 = a * reference[2] + b * reference[3];
    let c = anchor[1] as f32 + 1. - p[1];
    let d = p[1] - anchor[1] as f32;
    c * r_1 + d * r_2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbour_corner_upper_left_up() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(0, Direction::Up), 9900);
    }

    #[test]
    fn test_neighbour_corner_upper_left_down() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(0, Direction::Down), 100);
    }

    #[test]
    fn test_neighbour_corner_upper_left_left() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(0, Direction::Left), 99);
    }

    #[test]
    fn test_neighbour_corner_upper_left_right() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(0, Direction::Right), 1);
    }

    #[test]
    fn test_neighbour_corner_upper_right_up() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(99, Direction::Up), 9999);
    }
    #[test]
    fn test_neighbour_corner_upper_right_down() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(99, Direction::Down), 199);
    }

    #[test]
    fn test_neighbour_corner_upper_right_left() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(99, Direction::Left), 98);
    }

    #[test]
    fn test_neighbour_corner_upper_right_right() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(99, Direction::Right), 0);
    }

    #[test]
    fn test_neighbour_corner_lower_left_up() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9900, Direction::Up), 9800);
    }
    #[test]
    fn test_neighbour_corner_lower_left_down() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9900, Direction::Down), 0);
    }
    #[test]
    fn test_neighbour_corner_lower_left_left() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9900, Direction::Left), 9999);
    }

    #[test]
    fn test_neighbour_corner_lower_left_right() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9900, Direction::Right), 9901);
    }

    #[test]
    fn test_neighbour_corner_lower_right_up() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9999, Direction::Up), 9899);
    }
    #[test]
    fn test_neighbour_corner_lower_right_down() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9999, Direction::Down), 99);
    }
    #[test]
    fn test_neighbour_corner_lower_right_left() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9999, Direction::Left), 9998);
    }

    #[test]
    fn test_neighbour_corner_lower_right_right() {
        let m = Model::new_zeroed(100);
        assert_eq!(m.get_neighbour_index(9999, Direction::Right), 9900);
    }

    #[test]
    fn test_opposites() {
        assert_eq!(Direction::Up, Direction::Down.get_opposite());
        assert_eq!(Direction::Down, Direction::Up.get_opposite());
        assert_eq!(Direction::Left, Direction::Right.get_opposite());
        assert_eq!(Direction::Right, Direction::Left.get_opposite());
    }

    #[test]
    fn test_opposite_point_same() {
        let m = Model::new_zeroed(100);

        assert_eq!(
            123,
            m.get_neighbour_index(
                m.get_neighbour_index(123, Direction::Left),
                Direction::Right
            )
        );
        assert_eq!(
            0,
            m.get_neighbour_index(m.get_neighbour_index(0, Direction::Left), Direction::Right)
        );
        assert_eq!(
            99,
            m.get_neighbour_index(m.get_neighbour_index(99, Direction::Left), Direction::Right)
        );
        assert_eq!(
            9900,
            m.get_neighbour_index(
                m.get_neighbour_index(9900, Direction::Left),
                Direction::Right
            )
        );
        assert_eq!(
            9999,
            m.get_neighbour_index(
                m.get_neighbour_index(9999, Direction::Left),
                Direction::Right
            )
        );
    }

    #[test]
    fn test_tilt_angle() {
        let mut m = Model::new_zeroed(100);
        assert_eq!(m.get_tilt_angle(0), 0.);

        m.terrain_height[0] = 100.;
        assert_eq!(m.get_tilt_angle(0), 0.);

        let left = m.get_neighbour_index(0, Direction::Left);
        let right = m.get_neighbour_index(0, Direction::Right);
        let up = m.get_neighbour_index(0, Direction::Up);
        let down = m.get_neighbour_index(0, Direction::Down);

        m.terrain_height[left] = 10000.;
        assert!((m.get_tilt_angle(0).sin() - 1.).abs() < 1e-3);

        m.terrain_height[left] = 0.;
        m.terrain_height[right] = 10000.;
        assert!((m.get_tilt_angle(0).sin() - 1.).abs() < 1e-3);

        m.terrain_height[right] = 0.;
        m.terrain_height[up] = 10000.;
        assert!((m.get_tilt_angle(0).sin() - 1.).abs() < 1e-3);

        m.terrain_height[up] = 0.;
        m.terrain_height[down] = 10000.;
        assert!((m.get_tilt_angle(0).sin() - 1.).abs() < 1e-3);
    }
}
