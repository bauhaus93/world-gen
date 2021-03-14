use core::{
    Camera, Config, Float, Input, Player, Point3f, RenderInfo, Renderable, Rotatable, State,
    StateError, Translatable, Updatable,
};

use crate::{World, CHUNK_SIZE};

pub struct WorldState {
    camera: Camera,
    player: Player,
    world: World,
    mouse_sensitivity: f32,
    fly_mode: bool,
}

impl WorldState {
    pub fn new(config: &Config) -> Result<WorldState, StateError> {
        let mut camera = Camera::default();
        let player = Player::default();
        let world = World::new(&config).map_err(|e| StateError::Setup(e.to_string()))?;

        camera.set_far((CHUNK_SIZE * config.get_int_or_default("active_radius", 50)) as f32 * 8.);

        Ok(WorldState {
            camera: camera,
            player: player,
            world: world,
            mouse_sensitivity: config.get_float_or_default("mouse_sensitivity", 0.3),
            fly_mode: false,
        })
    }
}

impl State for WorldState {
    fn update(&mut self, input: &Input) -> Result<(), StateError> {
        if input.key_pressed("F1") {
            self.fly_mode = !self.fly_mode;
            if self.fly_mode {
                self.player.clear_momentum();
            }
        }
        if self.fly_mode {
            self.update_player_fly_mode(input)?;
        } else {
            self.update_player(input)?;
        }
        self.update_camera();
        self.update_world(input)?;
        Ok(())
    }

    fn render(&self) -> Result<(), StateError> {
        let mut render_info = RenderInfo::new(&self.camera);
        self.world.render(&mut render_info)?;
        Ok(())
    }
}

impl WorldState {
    fn update_camera(&mut self) {
        self.player.align_camera(&mut self.camera);
    }

    fn update_world(&mut self, input: &Input) -> Result<(), StateError> {
        self.world.set_center(self.player.get_translation());
        self.world.tick(input.get_time_passed())?;
        Ok(())
    }
    fn update_player(&mut self, input: &Input) -> Result<(), StateError> {
        self.update_player_direction(input);
        self.update_player_momentum(input);
        self.world.interact(&mut self.player);

        if input.key_pressed("F2") {
            self.player.mod_speed(0.25);
        }
        if input.key_pressed("F3") {
            self.player.mod_speed(-0.25);
        }
        self.player.tick(input.get_time_passed())?;
        Ok(())
    }

    fn update_player_fly_mode(&mut self, input: &Input) -> Result<(), StateError> {
        self.update_player_direction(input);
        let mut offset = Point3f::from_scalar(0.);
        if let Some(move_keys) = input.get_movement_keys_down() {
            if move_keys[0] {
                offset += self.player.get_direction();
            }
            if move_keys[1] {
                offset -= self.player.get_direction().cross(&Point3f::new(0., 0., 1.));
            }
            if move_keys[2] {
                offset -= self.player.get_direction();
            }
            if move_keys[3] {
                offset += self.player.get_direction().cross(&Point3f::new(0., 0., 1.));
            }
        }
        if input.key_pressed("SPACE") {
            offset += Point3f::new(0., 0., 1.);
        }
        if offset.length() > 0. {
            self.player
                .mod_translation(offset.as_normalized() * self.player.get_speed());
        }
        if input.key_pressed("F2") {
            self.player.mod_speed(0.25);
        }
        if input.key_pressed("F3") {
            self.player.mod_speed(-0.25);
        }
        self.player.tick(input.get_time_passed())?;
        Ok(())
    }

    fn update_player_direction(&mut self, input: &Input) {
        if input.has_mouse_delta() {
            let delta = input.get_mouse_delta();
            let offset = Point3f::new(-delta.0 as f32, delta.1 as f32, 0.);
            let rotation =
                offset * self.mouse_sensitivity * (input.get_time_passed() as Float / 1000.);
            self.player.mod_rotation(rotation);
        }
    }

    fn update_player_momentum(&mut self, input: &Input) {
        if !self.player.is_jumping() {
            if let Some(move_keys) = input.get_movement_keys_down() {
                self.player.add_move_momentum(&move_keys);
            }

            if input.key_pressed("SPACE") {
                self.player.jump(3.);
            }
        }
    }
}
