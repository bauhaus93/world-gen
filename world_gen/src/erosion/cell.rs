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
    pub fn set_terrain_height(&mut self, new_height: Float) {
        self.terrain_height = new_height;
    }
    pub fn get_terrain_height(&self) -> Float {
        self.terrain_height
    }
    pub fn mod_water(&mut self, amount: Float) {
        self.water_height += amount;
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
        self.velocity = new_velocity;
    }
    pub fn set_normal(&mut self, new_normal: Vector3<Float>) {
        self.normal = new_normal;
    }
    pub fn update_transport_capacity(&mut self, sediment_capacity_constant: Float) {
        let cosa = dot(self.normal, Vector3::new(0., 0., 1.));
        let sin_alpha = Float::max(0.01, sin(acos(cosa)));
        self.transport_capacacity = sediment_capacity_constant * sin_alpha * length(self.velocity);
    }
    pub fn apply_erosion_deposition(&mut self, dissolving_constant: Float, deposition_constant: Float) {
        if self.transport_capacacity > self.suspended_sediment {
            let dissolved_sediment = dissolving_constant * (self.transport_capacacity - self.suspended_sediment);
            self.terrain_height -= dissolved_sediment;
            self.suspended_sediment += dissolved_sediment;
        } else {
            let deposited_sediment = deposition_constant * (self.suspended_sediment - self.transport_capacacity);
            self.terrain_height += deposited_sediment;
            self.suspended_sediment -= deposited_sediment;
        }
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


