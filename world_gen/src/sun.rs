use std::f32::consts::PI;
use std::ops::Sub;

use glm::{ GenNum, Vector3 };

use utility::Float;
use graphics::transformation::create_direction;
use crate::WorldError;
use crate::traits::Updatable;

pub struct Sun {
    rotation_center: Vector3<Float>,
    center_distance: Float,
    rotation: Vector3<Float>,
    rotation_speed: Float
}

impl Sun {
    pub fn set_rotation_center(&mut self, mut new_center: Vector3<Float>) {
        new_center[2] = 0.;
        self.rotation_center = new_center;
    }
    pub fn set_day_length(&mut self, length_seconds: u32) {
        self.rotation_speed = calculate_rotation_speed(length_seconds);
    }
    pub fn calculate_position(&self) -> Vector3<Float> {
        (create_direction(self.rotation) * self.center_distance).sub(self.rotation_center)
    }
    #[allow(unused)]
    pub fn calculate_daytime(&self) -> f32 {
        match 12. + 24. * self.rotation[1] / (2. * PI) {
            t if t > 24. => t - 24.,
            t => t
        }
    }
    pub fn calculate_light_level(&self) -> f32 {
        match 1. - self.rotation[1] / PI {
            t if t < 0. => -t,
            t => t
        }
    }
}

impl Default for Sun {
    fn default() -> Sun {
        const DEFAULT_DAY_LEN: u32 = 60;
        Sun {
            rotation_center: Vector3::from_s(0.),
            center_distance: 10000.,
            rotation: Vector3::from_s(0.),
            rotation_speed: calculate_rotation_speed(DEFAULT_DAY_LEN)
        }
    }
}

impl Updatable for Sun {
    fn tick(&mut self, time_passed: u32) -> Result<(), WorldError> {
        self.rotation[1] += self.rotation_speed * time_passed as Float;
        if self.rotation[1] > 2. * PI {
            self.rotation[1] -= 2. * PI;
        }
        Ok(())
    }
}

// day length in seconds, returns rad/ms
fn calculate_rotation_speed(day_length: u32) -> f32 {
    debug_assert!(day_length != 0);
    2. * PI / (day_length * 1000) as f32
}
