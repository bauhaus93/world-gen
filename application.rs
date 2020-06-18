use glutin;

use crate::ApplicationError;
use core::traits::{RenderInfo, Rotatable, Translatable, Updatable};
use core::{Point3f, Camera, Config, Core, Float, Player};
use world::World;

pub struct Application {
    core: Core,
    camera: Camera,
    player: Player,
    world: World,
	mouse_sensitivity: f32
}

impl Application {
    pub fn new(config_path: &str) -> Result<Application, ApplicationError> {
        let config = Config::read(config_path)?;
        let core = Core::new(&config)?;
        let mut camera = Camera::default();
        let player = Player::default();
        let world = World::new(&config)?;

        camera.set_far(world.get_active_radius() * 8.);

        let app = Application {
            core: core,
            camera: camera,
            player: player,
            world: world,
			mouse_sensitivity: config.get_float_or_default("mouse_sensitivity", 0.3)
        };
        Ok(app)
    }

    pub fn run(mut self) -> Result<(), ApplicationError> {
        while !self.core.should_quit() {
            self.core.update()?;
            if !self.core.is_hibernating() {
                self.update_player()?;
                self.update_camera();
                self.update_world()?;

                let mut render_info = RenderInfo::new(&self.camera);
                self.core.render(&self.world, &mut render_info)?;
            }
        }
        Ok(())
    }

    fn update_player(&mut self) -> Result<(), ApplicationError> {
        self.update_player_direction();
        self.update_player_momentum();
        self.world.interact(&mut self.player)?;
		if self.core.key_pressed(glutin::VirtualKeyCode::F1) {
			self.player.mod_speed(0.25);
		}
		if self.core.key_pressed(glutin::VirtualKeyCode::F2) {
			self.player.mod_speed(-0.25);
		}

        self.player.tick(self.core.get_time_passed())?;
        Ok(())
    }

    fn update_camera(&mut self) {
        self.player.align_camera(&mut self.camera);
    }

    fn update_world(&mut self) -> Result<(), ApplicationError> {
        self.world.set_center(self.player.get_translation());
        self.world.tick(self.core.get_time_passed())?;
        Ok(())
    }

    fn update_player_direction(&mut self) {
        if self.core.has_mouse_delta() {
            let delta = self.core.get_mouse_delta();
            let offset = Point3f::new(-delta.0 as f32, delta.1 as f32, 0.);
            let rotation = offset * self.mouse_sensitivity * (self.core.get_time_passed() as Float / 1000.);
            self.player.mod_rotation(rotation);
            self.core.center_mouse();
        }
    }

    fn get_movement_keys(&self) -> Option<[bool; 4]> {
        let mut move_down: [bool; 4] = [false, false, false, false];
        move_down[0] = self.core.key_pressed(glutin::VirtualKeyCode::W);
        move_down[1] = self.core.key_pressed(glutin::VirtualKeyCode::A);
        move_down[2] = self.core.key_pressed(glutin::VirtualKeyCode::S);
        move_down[3] = self.core.key_pressed(glutin::VirtualKeyCode::D);

        if move_down.iter().any(|&e| e) {
            Some(move_down)
        } else {
            None
        }
    }

    fn update_player_momentum(&mut self) {
        if !self.player.is_jumping() {
            if let Some(move_keys) = self.get_movement_keys() {
                self.player.add_move_momentum(&move_keys);
            }

            if self.core.key_pressed(glutin::VirtualKeyCode::Space) {
                self.player.jump(6.);
            }
        }
    }
}
