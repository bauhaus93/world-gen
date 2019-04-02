use glm::{ Vector2, Vector3, GenNum, dot, sin, acos, length };

use utility::Float;
use super::direction::Direction;

pub struct Cell {
    terrain_height: Float,
    water_height: Float,
    normal: Vector3<Float>,
    outflow: [Float; 4],
    velocity: Vector2<Float>,
    transport_capacacity: Float,
    suspended_sediment: Float,
    transported_sediment: Float,
}

impl Cell {
    pub fn has_water(&self) -> bool {
        self.water_height > 0.
    }
    pub fn has_velocity(&self) -> bool {
        self.velocity[0].abs() > 0. ||
        self.velocity[1].abs() > 0.
    }
    pub fn set_terrain_height(&mut self, new_height: Float) {
        debug_assert!(!new_height.is_nan());
        self.terrain_height = new_height;
    }
    pub fn get_terrain_height(&self) -> Float {
        self.terrain_height
    }
    pub fn mod_water(&mut self, amount: Float) {
        debug_assert!(!amount.is_nan());
        self.water_height += amount;
    }
    pub fn set_water(&mut self, value: Float) {
        debug_assert!(!value.is_nan());
        self.water_height = value;
    }
    pub fn get_water_height(&self) -> Float {
        self.water_height
    }
    pub fn get_water_level(&self) -> Float {
        self.terrain_height + self.water_height
    }
    pub fn get_flow(&self, dir: Direction) -> Float {
        let index: usize = dir.into();
        self.outflow[index]
    }
    pub fn set_flow(&mut self, dir: Direction, new_flow: Float) {
        let index: usize = dir.into();
        self.outflow[index] = new_flow;
    }
    pub fn set_velocity(&mut self, new_velocity: Vector2<Float>) {
        debug_assert!(!new_velocity.x.is_nan());
        debug_assert!(!new_velocity.y.is_nan());
        self.velocity = new_velocity;
    }
    pub fn get_sediment_source_offset(&self, time_delta: Float) -> Vector2<Float> {
        -self.velocity * time_delta
    }
    pub fn set_normal(&mut self, new_normal: Vector3<Float>) {
        debug_assert!(!new_normal.x.is_nan() || !new_normal.y.is_nan() || !new_normal.z.is_nan());
        self.normal = new_normal;
    }
    pub fn get_suspended_sediment(&self) -> Float {
        self.suspended_sediment
    }
    pub fn set_transported_sediment(&mut self, transported_sediment: Float) {
        debug_assert!(!transported_sediment.is_nan());
        self.transported_sediment = transported_sediment;
    }

    pub fn update_transport_capacity(&mut self, sediment_capacity_constant: Float) {
        debug_assert!(self.water_height > 0.);
        debug_assert!(!length(self.velocity).is_nan());
        let cosa = dot(self.normal, Vector3::new(0., 0., 1.));
        let sin_alpha = Float::max(0.01, sin(acos(cosa)));
        self.transport_capacacity = sediment_capacity_constant * sin_alpha * length(self.velocity);
        debug_assert!(!self.transport_capacacity.is_nan());
    }
    pub fn apply_erosion_deposition(&mut self, dissolving_constant: Float, deposition_constant: Float) {
        debug_assert!(!self.transport_capacacity.is_nan());
        debug_assert!(!self.suspended_sediment.is_nan());
        if self.transport_capacacity > self.suspended_sediment {
            let dissolved_sediment = dissolving_constant * (self.transport_capacacity - self.suspended_sediment);
            debug_assert!(!dissolved_sediment.is_nan());
            self.terrain_height -= dissolved_sediment;
            self.suspended_sediment += dissolved_sediment;
        } else {
            let deposited_sediment = deposition_constant * (self.suspended_sediment - self.transport_capacacity);
            debug_assert!(!deposited_sediment.is_nan());
            self.terrain_height += deposited_sediment;
            self.suspended_sediment -= deposited_sediment;
        }
    }
    pub fn apply_sediment_transportation(&mut self) {
        debug_assert!(!self.transported_sediment.is_nan());
        self.suspended_sediment = self.transported_sediment;
    }
    pub fn apply_water_evaporation(&mut self, evaporation_factor: Float) {
        debug_assert!(evaporation_factor >= 0.);
        debug_assert!(evaporation_factor <= 1.);
        self.water_height *= evaporation_factor;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            terrain_height: 0.,
            water_height: 0.,
            normal: Vector3::from_s(0.),
            outflow: [0., 0., 0., 0.],
            velocity: Vector2::from_s(0.),
            transport_capacacity: 0.,
            suspended_sediment: 0.,
            transported_sediment: 0.,
        }
    }
}


