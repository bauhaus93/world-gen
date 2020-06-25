use rand::{rngs::SmallRng, Rng};
use std::ops::{Add, AddAssign};

use crate::HeightMap;
use core::{Point2f, Point3f, Seed};

type Flux = [f32; 4];

// TODO: Proper values for constants
const PIPE_AREA: f32 = 0.5;
const PIPE_LENGTH: f32 = 0.25;
const GRID_DISTANCE_X: f32 = 0.01;
const GRID_DISTANCE_Y: f32 = 0.01;
const GRAVITY: f32 = 0.5;
const DELTA_TIME: f32 = 1e-1;
const SEDIMENT_CAPACITY_CONSTANT: f32 = 0.4;
const DISSOLVING_CONSTANT: f32 = 5e-1;
const DEPOSITION_CONSTANT: f32 = 1e-1;
const EVAPORATION_CONSTANT: f32 = 0.1;
const MIN_TILT: f32 = 0.1;

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
    pub fn new_zeroed_from_ref(ref_model: &Model) -> Self {
        Self::new_zeroed(ref_model.size)
    }
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

    pub fn get_terrain_suspended_ratio(&self) -> f32 {
        self.get_total_suspended_sediment() / self.get_total_terrain_height()
    }

    pub fn run(mut self, count: usize, seed: Seed) -> Self {
        info!("Starting erosion simulation with {} turns", count);
        let mut rng: SmallRng = seed.into();
        let mut model_delta = Model::new_zeroed_from_ref(&self);

        for i in 0..count {
            self = self.run_once(&mut rng);
            if i % usize::max(1, (count / 20)) == 0 {
                info!(
                    "Progress: {:2}% | terrain height: {:.0} | susp sed: {:.2} | total water: {:.2}",
                    100 * i / count,
                    self.get_total_terrain_height(),
                    self.get_total_suspended_sediment(),
                    self.get_total_water()
                );
            }
        }
        self
    }

    fn run_once(mut self, rng: &mut impl Rng) -> Self {
        let mut delta = Model::new_zeroed_from_ref(&self);

        delta.rain(1., 10000, rng);

        delta.calculate_flux_delta(&self);
        self.apply_flux_delta(&delta);
        delta.calculate_flux_water_delta(&self);
        delta.calculate_velocity_field_delta(&self);
        self.apply_velocity_field_delta(&delta);

        delta.calculate_erosion_deposition_delta(&self);
        self.apply_erosion_deposition_delta(&delta);

        delta.calculate_sediment_transportation_delta(&self);
        self.apply_sediment_transportation_delta(&delta);

        delta.calculate_evaporation_delta(&self);

        self.apply_water_delta(&delta);
        self.apply_terrain_delta(&delta);

        self
    }

    pub fn rain(&mut self, total_amount: f32, drop_count: usize, rng: &mut impl Rng) {
        let drop_size = total_amount / drop_count as f32;
        for i in 0..drop_count {
            let index: usize = rng.gen_range(0, self.water_height.len());
            self.water_height[index] += drop_size;
        }
    }

    pub fn calculate_flux_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            for dir in Direction::as_slice().iter() {
                let nb = self.get_neighbour_index(i, *dir);
                let delta_height = self.terrain_height[i] + self.water_height[i]
                    - self.terrain_height[nb]
                    - self.water_height[nb];
                self.outflow_flux[i][dir.get_index()] =
                    DELTA_TIME * PIPE_AREA * ((GRAVITY * delta_height) / PIPE_LENGTH);
            }
        }
    }

    pub fn calculate_flux_water_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            let inflow = Direction::as_slice()
                .iter()
                .map(|d| m.outflow_flux[m.get_neighbour_index(i, *d)][d.get_opposite().get_index()])
                .fold(0., |acc, flux| acc + flux);
            let outflow = Direction::as_slice()
                .iter()
                .fold(0., |acc, dir| acc + m.outflow_flux[i][dir.get_index()]);
            let delta = (DELTA_TIME * (inflow - outflow)) / (GRID_DISTANCE_X * GRID_DISTANCE_Y);
            self.water_height[i] += delta;
        }
    }

    pub fn calculate_velocity_field_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            let flux_delta_left = self.outflow_flux[self.get_neighbour_index(i, Direction::Left)]
                [Direction::Right.get_index()]
                - self.outflow_flux[i][Direction::Left.get_index()];
            let flux_delta_right = self.outflow_flux[i][Direction::Right.get_index()]
                - self.outflow_flux[self.get_neighbour_index(i, Direction::Right)]
                    [Direction::Left.get_index()];
            let flux_delta_up = self.outflow_flux[self.get_neighbour_index(i, Direction::Up)]
                [Direction::Down.get_index()]
                - self.outflow_flux[i][Direction::Up.get_index()];
            let flux_delta_down = self.outflow_flux[i][Direction::Down.get_index()]
                - self.outflow_flux[self.get_neighbour_index(i, Direction::Down)]
                    [Direction::Up.get_index()];

            let flux_delta_x = (flux_delta_left + flux_delta_right) / 2.;
            let flux_delta_y = (flux_delta_up + flux_delta_down) / 2.;
            let u = flux_delta_x / f32::max(1e-3, (GRID_DISTANCE_Y * m.water_height[i])); // possible problem: not using average of water height between intermediate steps (instead of (d1+d2)/2, just using d2)
            let v = flux_delta_y / f32::max(1e-3, (GRID_DISTANCE_X * m.water_height[i])); // possible problem: not using average of water height between intermediate steps (instead of (d1+d2)/2, just using d2)
            self.velocity[i] = Point2f::new(
                f32::min(u, GRID_DISTANCE_X / DELTA_TIME),
                f32::min(v, GRID_DISTANCE_Y / DELTA_TIME),
            ) - m.velocity[i];
            debug_assert!(!self.velocity[i][0].is_infinite() && !self.velocity[i][0].is_nan());
            debug_assert!(!self.velocity[i][1].is_infinite() && !self.velocity[i][1].is_nan());
        }
    }

    pub fn calculate_erosion_deposition_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            let transport_capacity = SEDIMENT_CAPACITY_CONSTANT
                * f32::max(MIN_TILT, m.get_tilt_angle(i)).sin()
                * m.velocity[i].length();
            if transport_capacity > m.suspended_sediment[i] {
                let delta_suspendend_sediment =
                    DISSOLVING_CONSTANT * (transport_capacity - m.suspended_sediment[i]);
                self.terrain_height[i] -= delta_suspendend_sediment;
                self.suspended_sediment[i] += delta_suspendend_sediment;
            } else {
                let delta_deposited_sediment =
                    DEPOSITION_CONSTANT * (m.suspended_sediment[i] - transport_capacity);
                self.terrain_height[i] += delta_deposited_sediment;
                self.suspended_sediment[i] -= delta_deposited_sediment;
            }
        }
    }

    pub fn calculate_sediment_transportation_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            let mut source_pos = Point2f::new(
                (i % self.size) as f32 - m.velocity[i][0] * DELTA_TIME,
                (i / self.size) as f32 - m.velocity[i][1] * DELTA_TIME,
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
            self.suspended_sediment[i] = interpolate(
                source_pos,
                [
                    m.suspended_sediment[grid_ul_index],
                    m.suspended_sediment[grid_ur_index],
                    m.suspended_sediment[grid_dl_index],
                    m.suspended_sediment[grid_dr_index],
                ],
            ) - m.suspended_sediment[i];
        }
    }

    pub fn calculate_evaporation_delta(&mut self, m: &Model) {
        for i in 0..self.size * self.size {
            self.water_height[i] = ((self.water_height[i] + m.water_height[i])
                * (1. - EVAPORATION_CONSTANT * DELTA_TIME))
                - m.water_height[i];
        }
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
            _ => unreachable!(),
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

    fn apply_flux_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            for dir in Direction::as_slice().iter() {
                self.outflow_flux[i][dir.get_index()] = f32::max(
                    0.,
                    self.outflow_flux[i][dir.get_index()] + delta.outflow_flux[i][dir.get_index()],
                );
            }
            let scaling_factor = match self.outflow_flux[i].iter().fold(0., |acc, v| acc + v) {
                n if n > 0. => f32::min(
                    1.,
                    (self.water_height[i] * GRID_DISTANCE_X * GRID_DISTANCE_Y) / (n * DELTA_TIME),
                ),
                _ => 0.,
            };
            if scaling_factor > 0. {
                for dir in Direction::as_slice().iter() {
                    self.outflow_flux[i][dir.get_index()] *= scaling_factor;
                }
            }
        }
    }

    fn apply_velocity_field_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            self.velocity[i] += delta.velocity[i];
        }
    }

    fn apply_erosion_deposition_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            self.terrain_height[i] += delta.terrain_height[i];
            self.suspended_sediment[i] += delta.suspended_sediment[i];
        }
    }

    fn apply_sediment_transportation_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            self.suspended_sediment[i] += delta.suspended_sediment[i];
        }
    }

    fn apply_water_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            self.water_height[i] += delta.water_height[i];
        }
    }

    fn apply_terrain_delta(&mut self, delta: &Self) {
        for i in 0..self.size * self.size {
            self.terrain_height[i] += delta.terrain_height[i];
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
        HeightMap::from_list(self.size as i32, 1, self.terrain_height.as_slice())
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
