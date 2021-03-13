use crate::traits::{Rotatable, Translatable, Updatable};
use crate::Point3f;
use crate::{graphics::create_direction, Camera, Float, Model, UpdateError};

pub struct Player {
    model: Model,
    momentum: Point3f,
    forward: Point3f,
    speed: f32,
    jumping: bool,
}

impl Player {
    pub fn align_camera(&self, camera: &mut Camera) {
        let mut pos = self.get_translation();
        pos[2] += 3.;
        camera.set_translation(pos);
        camera.set_rotation(self.get_rotation());
    }

    pub fn is_jumping(&self) -> bool {
        self.jumping
    }

    pub fn toggle_jump(&mut self) {
        self.jumping = !self.jumping;
    }

    pub fn land(&mut self) {
        self.jumping = false;
    }

    pub fn jump(&mut self, force: Float) {
        self.jumping = true;
        let momentum_xy = self.momentum.as_xy();
        let jump_momentum = match momentum_xy.length() {
            mom_xy if mom_xy < 1e-3 => Point3f::new(0., 0., 1.) * force,
            _ => (momentum_xy.as_normalized() * self.speed).extend(force),
        };
        self.push(jump_momentum);
    }

    pub fn add_move_momentum(&mut self, directions: &[bool]) {
        debug_assert!(directions.len() >= 4);
        let mut move_offset = Point3f::new(0., 0., 0.);
        if directions[0] {
            move_offset += self.forward;
        }
        if directions[2] {
            move_offset -= self.forward;
        }
        if directions[1] || directions[3] {
            let up = Point3f::new(0., 0., 1.);

            if directions[1] {
                move_offset -= self.forward.cross(&up);
            }
            if directions[3] {
                let up = Point3f::new(0., 0., 1.);
                move_offset += self.forward.cross(&up);
            }
        }
        if move_offset.length() > 1e-3 {
            self.push(move_offset.as_normalized() * self.speed);
        }
    }

    pub fn apply_momentum(&mut self) {
        self.mod_translation(self.momentum);
    }

    pub fn update_forward(&mut self, forward: Point3f) {
        self.forward = forward;
    }

    pub fn move_z(&mut self, offset: Float) {
        self.mod_translation(Point3f::new(0., 0., offset));
    }

    pub fn set_z(&mut self, pos_z: Float) {
        let mut pos = self.get_translation();
        pos[2] = pos_z;
        self.set_translation(pos);
    }

    pub fn get_z(&self) -> f32 {
        self.model.get_translation()[2]
    }

    pub fn mod_speed(&mut self, amount: f32) {
        self.speed = f32::max(self.speed + amount, 1e-3);
    }

    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn get_direction(&self) -> Point3f {
        create_direction(self.get_rotation())
    }

    pub fn push(&mut self, additional_momentum: Point3f) {
        self.momentum += additional_momentum;
    }

    pub fn push_z(&mut self, additional_momentum_z: Float) {
        self.momentum[2] += additional_momentum_z;
    }

    pub fn clear_momentum(&mut self) {
        self.momentum = Point3f::from_scalar(0.);
    }

    pub fn clear_momentum_z(&mut self) {
        self.momentum[2] = 0.;
    }

    pub fn clear_momentum_neg_z(&mut self) {
        if self.momentum[2] < 0. {
            self.momentum[2] = 0.;
        }
    }
}

impl Default for Player {
    fn default() -> Player {
        let mut player = Player {
            model: Model::default(),
            momentum: Point3f::from_scalar(0.),
            forward: Point3f::from_scalar(0.),
            speed: 0.5,
            jumping: false,
        };
        player.set_translation(Point3f::new(0., 0., 200.));
        player.set_rotation(Point3f::new(45f32.to_radians(), 125f32.to_radians(), 0.));
        player
    }
}

impl Updatable for Player {
    fn tick(&mut self, _time_passed: u32) -> Result<(), UpdateError> {
        self.apply_momentum();
        if !self.jumping {
            self.clear_momentum();
        }
        Ok(())
    }
}

impl Translatable for Player {
    fn set_translation(&mut self, new_translation: Point3f) {
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Point3f {
        self.model.get_translation()
    }
}

impl Rotatable for Player {
    fn set_rotation(&mut self, new_rotation: Point3f) {
        const THRESHOLD: f32 = 0.01;
        const MIN_Y: Float = THRESHOLD;
        const MAX_Y: Float = std::f32::consts::PI as Float - THRESHOLD;
        const DOUBLE_PI: f32 = 2. * std::f32::consts::PI;
        let mut fixed_rotation = new_rotation;
        if fixed_rotation[0] >= DOUBLE_PI {
            fixed_rotation[0] -= DOUBLE_PI;
        } else if fixed_rotation[0] < 0. {
            fixed_rotation[0] += DOUBLE_PI;
        }
        if fixed_rotation[1] < MIN_Y {
            fixed_rotation[1] = MIN_Y;
        } else if fixed_rotation[1] > MAX_Y {
            fixed_rotation[1] = MAX_Y;
        }
        self.model.set_rotation(fixed_rotation);
    }
    fn get_rotation(&self) -> Point3f {
        self.model.get_rotation()
    }
}
