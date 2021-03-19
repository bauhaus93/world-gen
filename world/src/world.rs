use rand::rngs::StdRng;
use std::sync::Arc;

use crate::architect::Architect;
use crate::chunk::{ChunkManager, CHUNK_SIZE};
use crate::{Water, WorldError};
use core::graphics::GraphicsError;
use core::light::{Light, SceneLights};
use core::traits::{RenderInfo, Renderable, Rotatable, Scalable, Translatable, Updatable};
use core::{Config, ObjectManager, Player, Point3f, Seed, Skybox, Sun, Timer, UpdateError};

pub struct World {
    skybox: Skybox,
    water_surface: Water,
    sun: Sun,
    chunk_manager: ChunkManager,
    chunk_update_timer: Timer,
    decoration_timer: Timer,
    object_manager: ObjectManager,
    scene_lights: SceneLights,
    monkey_id: u32,
    center: Point3f,
    gravity: f32,
}

impl World {
    pub fn new(config: &Config) -> Result<World, WorldError> {
        let object_prototypes_path = config.get_str("object_prototype_path")?;
        let day_length = config.get_uint_or_default("day_length", 180);
        let gravity = config.get_float_or_default("gravity", 0.25);

        info!("Day length is {}s", day_length);
        info!("Gravity is {}", gravity);

        let seed = Seed::from_string("SEAS");
        info!("World seed = {}", seed);
        let mut rng: StdRng = seed.into();

        let mut object_manager = ObjectManager::from_yaml(&object_prototypes_path)?;
        let architect = Arc::new(Architect::from_seed(Seed::from_rng(&mut rng)));
        let chunk_manager = ChunkManager::new(architect, config)?;

        let monkey_id = object_manager.create_object("monkey", true)?;
        object_manager.mod_object(monkey_id, |o| {
            o.set_translation(Point3f::new(0., 0., 300.));
            o.set_scale(Point3f::from_scalar(10.));
        });

        let mut skybox = Skybox::new(config)?;
        skybox.scale((config.get_int_or_default("active_radius", 50) * CHUNK_SIZE * 2) as f32);

        let world = World {
            skybox: skybox,
            water_surface: Water::new(config)?,
            sun: Sun::with_day_length(day_length),
            chunk_manager: chunk_manager,
            chunk_update_timer: Timer::new(1000),
            decoration_timer: Timer::new(100),
            object_manager: object_manager,
            scene_lights: create_default_scene_lights(),
            monkey_id: monkey_id,
            center: Point3f::new(0., 0., 0.),
            gravity: gravity,
        };

        Ok(world)
    }

    pub fn interact(&mut self, player: &mut Player) {
        let player_pos = player.get_translation();

        let chunk_height = self.chunk_manager.get_height(player_pos);
        let player_pos_xy = player_pos.as_xy();
        let forward_xy = player.get_direction().as_xy().as_normalized();
        let forward_height = self
            .chunk_manager
            .get_height((player_pos_xy + forward_xy * player.get_speed()).extend(0.));
        let forward_z = forward_height - chunk_height;
        player.update_forward(forward_xy.extend(forward_z).as_normalized());

        let height_diff = player.get_z() - chunk_height;
        if height_diff > 0. {
            if height_diff > self.gravity && !player.is_jumping() {
                player.toggle_jump();
                player.set_z(chunk_height as f32);
            } else {
                player.push_z(f32::max(-self.gravity, -height_diff as f32));
            }
        } else {
            if player.is_jumping() {
                player.land();
            }
            player.set_z(chunk_height as f32);
        }
    }

    pub fn set_center(&mut self, pos: Point3f) {
        self.center = pos;
    }

    fn update_shader_resources(&mut self) -> Result<(), GraphicsError> {
        match self.scene_lights.get_light_mut("sun") {
            Some(sun_light) => {
                sun_light.set_world_pos(self.sun.calculate_position());
                sun_light.set_absolute_intensity(self.sun.calculate_intensity());
            }
            None => warn!("Could not get light source for sun"),
        }

        match self.scene_lights.get_light_mut("player") {
            Some(player_light) => {
                player_light.set_world_pos(self.center + Point3f::new(0., 0., 100.));
            }
            None => warn!("Could not get light source for player"),
        }

        let light_level = self.sun.calculate_intensity();
        let fog_color = Point3f::from_scalar(1. - (-light_level).exp());

        self.chunk_manager
            .update_shader_resources(self.center, fog_color, &self.scene_lights)?;
        self.skybox.update_light_level(light_level)?;

        self.water_surface
            .update_shader_resources(self.center, &self.scene_lights)?;

        Ok(())
    }

    /*fn get_chunk_by_world_pos(&self, world_pos: Point3f) -> Option<&Chunk> {
        self.chunks.get(&get_chunk_pos(world_pos))
    }*/
}

impl Renderable for World {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        /*self.surface_texture.activate();
        info.push_shader(self.surface_shader_program.clone());

        self.chunks.values().try_for_each(|c| c.render(info))?;


        info.pop_shader();

        self.surface_texture.deactivate();*/
        // self.object_manager.render(info)?;
        self.chunk_manager.render(info)?;
        self.water_surface.render(info)?;
        self.skybox.render(info)?;
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        self.chunk_manager.tick(time_passed)?;
        self.decoration_timer.tick(time_passed)?;
        if self.chunk_update_timer.fires() {
            self.chunk_manager
                .request(self.center)
                .map_err(|e| UpdateError::Internal(e.to_string()))?;
        }

        if self.decoration_timer.fires() {}

        self.skybox.set_translation(self.center);
        self.sun.set_rotation_center(self.center);
        self.sun.tick(time_passed)?;
        self.water_surface.set_translation(self.center);
        self.water_surface.tick(time_passed)?;

        let sun_pos = self.sun.calculate_position();
        let center = self.center;
        self.object_manager.mod_object(self.monkey_id, |o| {
            o.set_translation((sun_pos - center).as_normalized() * 200. + center);
            o.mod_rotation(Point3f::new(0., 0., 0.1));
        });

        self.update_shader_resources()?;
        self.chunk_update_timer.tick(time_passed)?;
        Ok(())
    }
}

fn create_default_scene_lights() -> SceneLights {
    let mut scene_lights = SceneLights::default();

    let mut sun_light = Light::default();
    sun_light.set_color(Point3f::from_scalar(1.));
    sun_light.set_absolute_intensity(1e8);
    sun_light.set_ambient_intensity(0.1);
    sun_light.set_diffuse_intensity(1.);
    sun_light.set_specular_intensity(0.4);
    sun_light.set_specular_shininess(2.);

    if !scene_lights.add_light("sun", sun_light) {
        warn!("Could not add sun light source to scene_lights");
    }

    let mut player_light = Light::default();
    player_light.set_color(Point3f::new(1., 1., 1.));
    player_light.set_absolute_intensity(1e4);
    player_light.set_ambient_intensity(0.);
    player_light.set_diffuse_intensity(0.5);
    player_light.set_specular_intensity(0.1);
    player_light.set_specular_shininess(2.);

    if !scene_lights.add_light("player", player_light) {
        warn!("Could not add player light source to scene_lights");
    }
    scene_lights
}
