use glm::Vector3;

use utility::Float;
use graphics::create_direction;
use crate::{ Model, Camera };
use crate::traits::{ Translatable, Rotatable };

pub struct Player {
    model: Model,
    speed: f32
}

impl Player {
    pub fn align_camera(&self, camera: &mut Camera) {
        let mut pos = self.get_translation();
        pos.z += 10.;
        camera.set_translation(pos);
        camera.set_rotation(self.get_rotation());
    }
    pub fn move_by_speed(&mut self, normalized_offset: Vector3<Float>) {
        self.mod_translation(normalized_offset * self.speed);
    }

    pub fn mod_speed(&mut self, amount: f32) {
        self.speed += amount;
    }

    pub fn get_direction(&self) -> Vector3<Float> {
        create_direction(self.get_rotation())
    }
}

impl Default for Player {
    fn default() -> Player {
        let mut player = Player {
            model: Model::default(),
            speed: 2.
        };
        player.set_translation(Vector3::new(0., 0., 200.));
        player.set_rotation(Vector3::new(45f32.to_radians(), 125f32.to_radians(), 0.));
        player
    }
}

impl Translatable for Player {
    fn set_translation(&mut self, new_translation: Vector3<Float>) {
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.model.get_translation()
    }
}

impl Rotatable for Player {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>) {
        const THRESHOLD: f32 = 0.01;
        const MIN_Y: Float = THRESHOLD;
        const MAX_Y: Float = std::f32::consts::PI as Float - THRESHOLD;
        const DOUBLE_PI: Float = 2. * std::f32::consts::PI as Float;
        let mut fixed_rotation = new_rotation;
        if fixed_rotation.x >= DOUBLE_PI {
            fixed_rotation.x -= DOUBLE_PI;
        } else if fixed_rotation.x < 0. {
            fixed_rotation.x += DOUBLE_PI;
        }
        if fixed_rotation.y < MIN_Y {
            fixed_rotation.y = MIN_Y;
        } else if fixed_rotation.y > MAX_Y {
            fixed_rotation.y = MAX_Y;
        }
        self.model.set_rotation(fixed_rotation);
    }
    fn get_rotation(&self) -> Vector3<Float> {
        self.model.get_rotation()
    }
}