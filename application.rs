use glutin;
use glm::Vector3;

use core::{ Core, CoreError, Config, Camera, Player, Float };
use core::traits::{Updatable, Renderable, Rotatable};
use world::World;
use crate::ApplicationError;

pub struct Application {
	core: Core,
	camera: Camera,
	player: Player,
	world: World
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
			world: world
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
				// self.core.render(&self.world)?;
			}
		}
		Ok(())
	}

	fn update_player(&mut self) -> Result<(), ApplicationError> {
		self.update_player_direction();
		self.update_player_momentum();
		self.world.interact(&mut self.player);

		self.player.tick(self.core.get_time_passed())?;
		Ok(())
	}

	fn update_camera(&mut self) {
		self.player.align_camera(&mut self.camera);
	}

	fn update_world(&mut self) -> Result<(), ApplicationError> {
		self.world.tick(self.core.get_time_passed())?;
		Ok(())
	}

	fn update_player_direction(&mut self) {
		if self.core.has_mouse_delta() {
			let delta = self.core.get_mouse_delta();
			let offset = Vector3::new(-delta.0 as Float, delta.1 as Float, 0.);
			let rotation = offset * 0.025 * (self.core.get_time_passed() as Float / 1000.);
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
		} else  {
			None
		}
	}

    fn update_player_momentum(&mut self) {
        if !self.player.is_jumping() {
			if let Some(move_keys) = self.get_movement_keys() {
				self.player.add_move_momentum(&move_keys);
			}

			if self.core.key_pressed(glutin::VirtualKeyCode::Space) {
				self.player.jump(4.);
			}
        }
    }
}