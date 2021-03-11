use std::collections::BTreeMap;
use std::iter::repeat;
use std::rc::Rc;
use std::sync::Arc;

use rand::rngs::StdRng;

use crate::architect::Architect;
use crate::chunk::{chunk_size::get_chunk_pos, Chunk, ChunkLoader, CHUNK_SIZE};
use crate::noise::presets::get_default_noise;
use crate::{Water, WorldError};
use core::graphics::{GraphicsError, ShaderProgram, ShaderProgramBuilder};
use core::light::{Light, SceneLights};
use core::traits::{RenderInfo, Renderable, Rotatable, Scalable, Translatable, Updatable};
use core::{
    Config, ObjectManager, Player, Point2i, Point3f, Seed, Skybox, Sun, Timer, UpdateError,
};

pub struct World {
    surface_shader_program: Rc<ShaderProgram>,
    skybox: Skybox,
    water_surface: Water,
    sun: Sun,
    architect: Arc<Architect>,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<Point2i, Chunk>,
    chunk_update_timer: Timer,
    chunk_build_stats_timer: Timer,
    lod_near_radius: i32,
    lod_far_radius: i32,
    active_chunk_radius: i32,
    object_manager: ObjectManager,
    scene_lights: SceneLights,
    monkey_id: u32,
    center: Point3f,
    gravity: f32,
    size: Option<Point2i>,
}

impl World {
    pub fn new(config: &Config) -> Result<World, WorldError> {
        let object_prototypes_path = config.get_str("object_prototype_path")?;
        let day_length = config.get_uint_or_default("day_length", 180);
        let gravity = config.get_float_or_default("gravity", 0.25);

        let surface_shader_program = load_surface_shader(config)?;

        let (near_radius, far_radius, active_radius) = get_chunk_radii(config);

        info!("Day length is {}s", day_length);
        info!("Gravity is {}", gravity);

        //let seed = Seed::from_entropy();
        let seed = Seed::from_byte_string("598FA4C6E7911AAA26DA2327D2D183B2")
            .unwrap_or_else(|| Seed::from_entropy());
        info!("World seed = {}", seed);
        let mut rng: StdRng = seed.into();

        let mut object_manager = ObjectManager::from_yaml(&object_prototypes_path)?;
        let architect = Arc::new(Architect::from_noise(get_default_noise(Seed::from_rng(
            &mut rng,
        ))));

        let chunk_loader = ChunkLoader::new(architect.clone());

        let monkey_id = object_manager.create_object("monkey", true)?;
        object_manager.mod_object(monkey_id, |o| {
            o.set_translation(Point3f::new(0., 0., 300.));
            o.set_scale(Point3f::from_scalar(10.));
        });

        let mut world = World {
            surface_shader_program: Rc::new(surface_shader_program),
            skybox: Skybox::new(config)?,
            water_surface: Water::new(config)?,
            sun: Sun::with_day_length(day_length),
            architect: architect,
            chunk_loader: chunk_loader,
            chunks: BTreeMap::new(),
            chunk_update_timer: Timer::new(500),
            chunk_build_stats_timer: Timer::new(5000),
            lod_near_radius: near_radius,
            lod_far_radius: far_radius,
            active_chunk_radius: active_radius,
            object_manager: object_manager,
            scene_lights: create_default_scene_lights(),
            monkey_id: monkey_id,
            center: Point3f::new(0., 0., 0.),
            gravity: gravity,
            size: None,
        };

        world.update_skybox_size();

        world.chunk_loader.start(8);
        world.request_chunks()?;

        Ok(world)
    }

    pub fn get_active_radius(&self) -> f32 {
        (self.active_chunk_radius * CHUNK_SIZE * 8) as f32
    }

    pub fn interact(&mut self, player: &mut Player) {
        let player_pos = player.get_translation();

        let chunk_height = match self.get_chunk_by_world_pos(player_pos) {
            Some(chunk) => {
                let player_pos_xy = player_pos.as_xy();
                let height = chunk.get_height(player_pos_xy);
                let forward_xy = player.get_direction().as_xy().as_normalized();
                let forward_height = chunk.get_height(player_pos_xy + forward_xy);
                let forward_z = forward_height - height;

                player.update_forward(forward_xy.extend(forward_z));
                height
            }
            None => {
                trace!("Player not on any chunk!");
                player.update_forward(player.get_direction());
                player_pos[2]
            }
        };

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

    pub fn request_chunks(&mut self) -> Result<(), WorldError> {
        let mut request_list: Vec<(Point2i, u8)> = Vec::new();
        let player_chunk_pos = get_chunk_pos(self.center);

        for r in 0..self.active_chunk_radius {
            let rad_iter_x = (-r..r + 1)
                .zip(repeat(r))
                .chain((-r..r + 1).zip(repeat(-r)));
            let rad_iter_y = repeat(r).zip(-r..r + 1).chain(repeat(-r).zip(-r..r + 1));
            for pos in rad_iter_x.chain(rad_iter_y) {
                if let Some(pos_lod) =
                    self.should_load_chunk(Point2i::new(pos.0, pos.1), player_chunk_pos)
                {
                    request_list.push(pos_lod);
                }
            }
        }
        self.chunk_loader.request(&request_list)?;
        trace!("Requested chunks: {}", request_list.len());
        Ok(())
    }

    fn unload_distant_chunks(&mut self) {
        let mut unload_list = Vec::new();
        let cam_pos = get_chunk_pos(self.center);
        for chunk_pos in self.chunks.keys() {
            let vec = [cam_pos[0] - chunk_pos[0], cam_pos[1] - chunk_pos[1]];
            let distance = vec[0] * vec[0] + vec[1] * vec[1];
            if distance >= self.active_chunk_radius * self.active_chunk_radius {
                unload_list.push(*chunk_pos);
            }
        }
        trace!("Unloading {} chunks", unload_list.len());

        let mut object_list = Vec::new();
        for pos in unload_list {
            match self.chunks.get(&pos) {
                Some(c) => object_list.extend(c.get_objects()),
                None => {}
            }
            self.chunks.remove(&pos);
        }
        self.object_manager.unload_by_list(&object_list);
    }

    fn get_finished_chunks(&mut self) -> Result<(), WorldError> {
        let mut finished_chunks = self.chunk_loader.get(200)?;
        if finished_chunks.len() > 0 {
            for chunk in finished_chunks.values_mut() {
                if chunk.get_lod() <= 1 {
                    chunk.load_objects(&mut self.object_manager, &self.architect)?;
                }
            }
            self.chunks.extend(finished_chunks);
        }
        Ok(())
    }

    pub fn count_loaded_vertices(&self) -> u32 {
        let mut vertex_count = 0;
        self.chunks
            .iter()
            .for_each(|(_, c)| vertex_count += c.get_vertex_count());
        vertex_count
    }

    pub fn set_center(&mut self, pos: Point3f) {
        self.center = pos;
    }

    fn should_load_chunk(&self, rel_pos: Point2i, player_pos: Point2i) -> Option<(Point2i, u8)> {
        let abs_pos = player_pos + rel_pos;
        if let Some(size) = self.size {
            if abs_pos[0] < 0 || abs_pos[1] < 0 || abs_pos[0] > size[0] || abs_pos[1] > size[1] {
                return None;
            }
        }

        {
            let distance = rel_pos.get_length() as i32;
            if distance < self.active_chunk_radius {
                let lod = self.lod_by_chunk_distance(distance);
                let chunk_pos = player_pos + rel_pos;
                match self.chunks.get(&chunk_pos) {
                    Some(c) => {
                        let old_lod = c.get_lod();
                        if lod != old_lod && (lod < 2 || old_lod < 2) {
                            Some((chunk_pos, lod))
                        } else {
                            None
                        }
                    }
                    None => Some((chunk_pos, lod)),
                }
            } else {
                None
            }
        }
    }

    fn lod_by_chunk_distance(&self, distance: i32) -> u8 {
        if distance < self.lod_near_radius {
            0
        } else if distance < self.lod_far_radius {
            1
        } else {
            2
        }
    }

    fn update_skybox_size(&mut self) {
        self.skybox
            .scale((self.active_chunk_radius * CHUNK_SIZE * 2) as f32);
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

        self.surface_shader_program.use_program();
        self.surface_shader_program
            .set_resource_vec3("view_pos", &self.center.as_glm())?;

        self.scene_lights
            .update_lights_for_shader(&self.surface_shader_program)?;

        let light_level = self.sun.calculate_intensity();
        let fog_color = Point3f::from_scalar(1. - (-light_level).exp());
        self.surface_shader_program
            .set_resource_vec3("fog_color", &fog_color.as_glm())?;
        self.skybox.update_light_level(light_level)?;

        self.water_surface
            .update_shader_resources(self.center, &self.scene_lights)?;

        Ok(())
    }

    fn get_chunk_by_world_pos(&self, world_pos: Point3f) -> Option<&Chunk> {
        self.chunks.get(&get_chunk_pos(world_pos))
    }
}

impl Renderable for World {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        //self.surface_texture.activate();
        info.push_shader(self.surface_shader_program.clone());

        self.chunks.values().try_for_each(|c| c.render(info))?;

        self.object_manager.render(info)?;

        info.pop_shader();

        //self.surface_texture.deactivate();
        self.skybox.render(info)?;
        self.water_surface.render(info)?;
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        if self.chunk_update_timer.fires() {
            self.get_finished_chunks()
                .map_err(|e| UpdateError::Internal(e.to_string()))?;
            self.unload_distant_chunks();
            self.request_chunks()
                .map_err(|e| UpdateError::Internal(e.to_string()))?;
            let rem_count = self
                .object_manager
                .unload_distant(self.center, (self.lod_far_radius * (1 + CHUNK_SIZE)) as f32);
            if rem_count > 0 {
                info!("Removed {} objects", rem_count);
            }
        }
        if self.chunk_build_stats_timer.fires() {
            info!(
                "Avg chunk build time = {:.2} ms, active chunks = {}, active objects = {}",
                self.chunk_loader.get_avg_build_time(),
                self.chunks.len(),
                self.object_manager.count_active_objects()
            );
        }

        self.skybox.set_translation(self.center);
        self.sun.set_rotation_center(self.center);
        self.sun.tick(time_passed)?;
        self.water_surface.set_translation(self.center);
        self.water_surface.tick(time_passed)?;

        let sun_pos = self.sun.calculate_position();
        let center = self.center;
        self.object_manager.mod_object(self.monkey_id, |o| {
            o.set_translation((sun_pos - center).as_normalized() * 200. + center);
            o.mod_rotation(Point3f::new(0., 0., 0.25));
        });

        self.update_shader_resources()?;
        self.chunk_update_timer.tick(time_passed)?;
        self.chunk_build_stats_timer.tick(time_passed)?;
        Ok(())
    }
}

fn get_chunk_radii(config: &Config) -> (i32, i32, i32) {
    let active_radius = config.get_int_or_default("active_radius", 40);
    let far_radius = i32::min(
        config.get_int_or_default("far_radius", 3 * active_radius / 2),
        active_radius,
    );
    let near_radius = i32::min(
        config.get_int_or_default("near_radius", active_radius / 3),
        far_radius,
    );
    (near_radius, far_radius, active_radius)
}

fn load_surface_shader(config: &Config) -> Result<ShaderProgram, WorldError> {
    let surface_shader_dir = config.get_str("surface_shader_dir")?.to_owned();
    let surface_shader_program = ShaderProgramBuilder::new()
        .add_vertex_shader((surface_shader_dir.clone() + "/VertexShader.glsl").as_str())
        .add_fragment_shader((surface_shader_dir + "/FragmentShader.glsl").as_str())
        //.add_resource("texture_array")
        .add_resource("mvp")
        .add_resource("model")
        .add_resource("view_pos")
        .add_resource("fog_color")
        .add_resource("active_lights")
        .add_resource("scene_lights[0].color")
        .add_resource("scene_lights[0].world_pos")
        .add_resource("scene_lights[0].absolute_intensity")
        .add_resource("scene_lights[0].ambient_intensity")
        .add_resource("scene_lights[0].diffuse_intensity")
        .add_resource("scene_lights[0].specular_intensity")
        .add_resource("scene_lights[0].specular_shininess")
        .add_resource("scene_lights[1].color")
        .add_resource("scene_lights[1].world_pos")
        .add_resource("scene_lights[1].absolute_intensity")
        .add_resource("scene_lights[1].ambient_intensity")
        .add_resource("scene_lights[1].diffuse_intensity")
        .add_resource("scene_lights[1].specular_intensity")
        .add_resource("scene_lights[1].specular_shininess")
        .finish()?;
    // setting texture slot to 0
    /*if let Err(e) = surface_shader_program.set_resource_integer("texture_array", 0) {
        return Err(GraphicsError::from(e).into());
    }*/
    Ok(surface_shader_program)
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
