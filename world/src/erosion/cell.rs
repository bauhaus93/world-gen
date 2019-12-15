use glm::{acos, dot, length, sin, GenNum, Vector2, Vector3};

use super::{Direction, Parameter};

#[derive(Clone)]
pub struct Cell {
    pos: [i32; 2],
    terrain_height: f64,
    water_height: f64,
    normal: Vector3<f64>,
    outflow: [f64; 4],
    velocity: Vector2<f64>,
    transport_capacity: f64,
    suspended_sediment: f64,
    transported_sediment: f64,
}

impl Cell {

    pub fn new(pos: [i32; 2]) -> Self {
        let mut cell = Cell::default();
        cell.pos = pos;
        cell
    }

    pub fn get_total_height_diff(&self, other: &Cell) -> f64 {
        self.get_total_height() - other.get_total_height()
    }
    pub fn get_pos(&self) -> &[i32; 2] {
        &self.pos
    }
    pub fn has_water(&self) -> bool {
        self.water_height > 0.
    }
    pub fn has_velocity(&self) -> bool {
        self.velocity[0].abs() > 0. || self.velocity[1].abs() > 0.
    }
    pub fn set_terrain_height(&mut self, new_height: f64) {
        debug_assert!(!new_height.is_nan());
        self.terrain_height = new_height;
    }
    pub fn get_terrain_height(&self) -> f64 {
        self.terrain_height
    }
    pub fn mod_water(&mut self, amount: f64) {
        debug_assert!(!amount.is_nan());
        self.water_height += amount;
    }
    pub fn set_water(&mut self, value: f64) {
        debug_assert!(!value.is_nan());
        self.water_height = value;
    }
    pub fn get_water_height(&self) -> f64 {
        self.water_height
    }
    pub fn get_total_height(&self) -> f64 {
        self.terrain_height + self.water_height
    }
    pub fn get_flow(&self, dir: Direction) -> f64 {
        let index: usize = dir.into();
        self.outflow[index]
    }
    pub fn set_flow(&mut self, dir: Direction, new_flow: f64) {
        let index: usize = dir.into();
        self.outflow[index] = new_flow;
    }
    pub fn set_velocity(&mut self, new_velocity: Vector2<f64>) {
        debug_assert!(!new_velocity.x.is_nan());
        debug_assert!(!new_velocity.y.is_nan());
        self.velocity = new_velocity;
    }
    pub fn set_normal(&mut self, new_normal: Vector3<f64>) {
        debug_assert!(!new_normal.x.is_nan() || !new_normal.y.is_nan() || !new_normal.z.is_nan());
        self.normal = new_normal;
    }
    pub fn get_suspended_sediment(&self) -> f64 {
        self.suspended_sediment
    }
    pub fn set_transported_sediment(&mut self, transported_sediment: f64) {
        debug_assert!(!transported_sediment.is_nan());
        self.transported_sediment = transported_sediment;
    }

    pub fn calculate_flow(&self, dir: Direction, neighbour: &Cell, params: &Parameter) -> f64 {
        let height_diff = self.get_total_height_diff(neighbour);
        let flow_delta = height_diff * params.get_flow_factor();
        f64::max(0., self.get_flow(dir) + flow_delta)
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            pos: [0, 0],
            terrain_height: 0.,
            water_height: 0.,
            normal: Vector3::from_s(0.),
            outflow: [0., 0., 0., 0.],
            velocity: Vector2::from_s(0.),
            transport_capacity: 0.,
            suspended_sediment: 0.,
            transported_sediment: 0.,
        }
    }
}
