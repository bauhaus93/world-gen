use core::{
    Camera, Config, Float, Input, Player, Point3f, RenderInfo, Renderable, Rotatable, State,
    StateError, Translatable, Updatable,
};

use crate::World;

pub struct WorldState {
    camera: Camera,
    player: Player,
    world: World,
    mouse_sensitivity: f32,
}

impl WorldState {
    pub fn new(config: &Config) -> Result<WorldState, StateError> {
        let mut camera = Camera::default();
        let player = Player::default();
        let world = World::new(&config).map_err(|e| StateError::Setup(e.to_string()))?;

        camera.set_far(world.get_active_radius() * 8.);

        Ok(WorldState {
            camera: camera,
            player: player,
            world: world,
            mouse_sensitivity: config.get_float_or_default("mouse_sensitivity", 0.3),
        })
    }
}

impl State for WorldState {
    fn update(&mut self, input: &Input) -> Result<(), StateError> {
        self.update_player(input)?;
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
        if input.key_pressed("F1") {
            self.player.mod_speed(0.25);
        }
        if input.key_pressed("F2") {
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
