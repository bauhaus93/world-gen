use glm::{ Vector2, Vector3, GenNum };

use utility::Float;
use super::direction::{ Direction };

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