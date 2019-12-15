use glm::{dot, length, GenNum, Vector2, Vector3};

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
}

impl Cell {

    pub fn new(pos: [i32; 2]) -> Self {
        let mut cell = Cell::default();
        cell.pos = pos;
        cell
    }

	pub fn mod_height(&mut self, delta: f64) {
		self.terrain_height += delta;
	}
	pub fn mod_suspended_sediment(&mut self, delta: f64) {
		self.suspended_sediment += delta;
		// debug_assert!(self.suspended_sediment >= 0.);
	}

    pub fn get_total_height_diff(&self, other: &Cell) -> f64 {
        self.get_total_height() - other.get_total_height()
    }
    pub fn get_pos(&self) -> &[i32; 2] {
        &self.pos
    }
	pub fn get_normal(&self) -> Vector3<f64> {
		self.normal
	}
    pub fn has_water(&self) -> bool {
        self.water_height > 0.
    }
    pub fn has_velocity(&self) -> bool {
        self.velocity[0].abs() > 0. || self.velocity[1].abs() > 0.
    }
	pub fn get_velocity(&self) -> Vector2<f64> {
		self.velocity
	}
	pub fn get_speed(&self) -> f64 {
		length(self.velocity)
	}
    pub fn set_terrain_height(&mut self, new_height: f64) {
        debug_assert!(!new_height.is_nan());
        self.terrain_height = new_height;
    }
    pub fn get_terrain_height(&self) -> f64 {
        self.terrain_height
    }
	pub fn get_tilt(&self) -> f64 {
		1. - dot(Vector3::new(0., 0., 1.), self.normal)
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
        self.outflow[usize::from(dir)]
    }
    pub fn set_flow(&mut self, dir: Direction, new_flow: f64) {
        self.outflow[usize::from(dir)] = new_flow;
    }

	pub fn set_transport_capacity(&mut self, capacity: f64) {
		self.transport_capacity = capacity;
	}
	pub fn set_suspended_sediment(&mut self, sediment: f64) {
		self.suspended_sediment = sediment;
	}
	pub fn get_transport_capacity(&self) -> f64 {
		self.transport_capacity
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
	pub fn get_pos_vec(&self) -> Vector2<f64> {
		Vector2::new(self.pos[0] as f64, self.pos[1] as f64)
	}

	pub fn get_sediment_source(&self, time_delta: f64) -> Vector2<f64> {
		self.get_pos_vec() - self.get_velocity() * time_delta
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
        }
    }
}
