use std::collections::BTreeMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use rand::rngs::SmallRng;

use crate::architect::Architect;
use crate::chunk::{chunk_size::get_chunk_pos, Chunk, ChunkLoader, CHUNK_SIZE};
use crate::surface::SurfaceTexture;
use crate::WorldError;
use core::format::format_number;
use core::graphics::{GraphicsError, ShaderProgram, ShaderProgramBuilder};
use core::traits::{RenderInfo, Renderable, Rotatable, Scalable, Translatable, Updatable};
use core::{
    Config, Object, ObjectManager, Player, Point2i, Point3f, Seed, Skybox, Sun, Timer, UpdateError,
};

pub struct World {
    surface_texture: SurfaceTexture,
    surface_shader_program: Rc<ShaderProgram>,
    skybox: Skybox,
    sun: Sun,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<Point2i, Chunk>,
    chunk_update_timer: Timer,
    chunk_build_stats_timer: Timer,
    lod_near_radius: i32,
    lod_far_radius: i32,
    active_chunk_radius: i32,
    last_chunk_load: Point2i,
    #[allow(dead_code)]
    object_manager: Arc<ObjectManager>,
    test_monkey: Object,
    center: Point3f,
    gravity: f32,
    size: Point2i,
}

impl World {
    pub fn new(config: &Config) -> Result<World, WorldError> {
        let object_prototypes_path = config.get_str("object_prototype_path")?;
        let day_length = config.get_uint_or_default("day_length", 180);
        let skybox_img_path = config.get_str("skybox_img_path")?;
        let surface_texture_info_path = config.get_str("surface_info_path")?;
        let gravity = config.get_float_or_default("gravity", 0.25);

        let surface_shader_program = load_surface_shader(config)?;
        let surface_texture = SurfaceTexture::load(surface_texture_info_path)?;

        let (near_radius, far_radius, active_radius) = get_chunk_radii(config);

        info!("Day length is {}s", day_length);
        info!("Gravity is {}", gravity);

        let seed = Seed::from_entropy();
        info!("World seed = {}", seed);
        let mut rng: SmallRng = seed.into();

        let world_size = Point2i::from_scalar(8);

        let object_manager = Arc::new(ObjectManager::from_yaml(&object_prototypes_path)?);
        /*let architect = Architect::from_noise(
            get_standard_noise(Seed::from_rng(&mut rng)),
            (world_size + 2) * CHUNK_SIZE,
            surface_texture.get_terrain_set(),
        );*/
        let architect = Architect::from_file(
            Path::new("heightmap.dat"),
            surface_texture.get_terrain_set(),
        )
        .expect("Could find file");

        let chunk_loader = ChunkLoader::new(&mut rng, architect, object_manager.clone());

        let mut test_monkey = object_manager.create_object("monkey")?;
        test_monkey.set_translation(Point3f::new(0., 0., 400.));
        test_monkey.set_scale(Point3f::from_scalar(10.));

        let mut world = World {
            surface_texture: surface_texture,
            surface_shader_program: Rc::new(surface_shader_program),
            skybox: Skybox::new(skybox_img_path)?,
            sun: Sun::with_day_length(day_length),
            chunk_loader: chunk_loader,
            chunks: BTreeMap::new(),
            chunk_update_timer: Timer::new(500),
            chunk_build_stats_timer: Timer::new(5000),
            lod_near_radius: near_radius,
            lod_far_radius: far_radius,
            active_chunk_radius: active_radius,
            last_chunk_load: Point2i::default(),
            object_manager: object_manager,
            test_monkey: test_monkey,
            center: Point3f::new(0., 0., 0.),
            gravity: gravity,
            size: world_size,
        };

        world.update_skybox_size();

        world.chunk_loader.start(8);
        world.request_chunks()?;

        Ok(world)
    }

    pub fn get_active_radius(&self) -> f32 {
        (self.active_chunk_radius * CHUNK_SIZE * 8) as f32
    }

    pub fn interact(&mut self, player: &mut Player) -> Result<(), WorldError> {
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

        Ok(())
    }

    pub fn request_chunks(&mut self) -> Result<(), WorldError> {
        let mut request_list: Vec<(Point2i, u8)> = Vec::new();
        let player_chunk_pos = get_chunk_pos(self.center);
        for y in -self.active_chunk_radius..self.active_chunk_radius + 1 {
            for x in -self.active_chunk_radius..self.active_chunk_radius + 1 {
                let p = Point2i::new(x, y);
                if let Some(pos_lod) = self.should_load_chunk(p, player_chunk_pos) {
                    request_list.push(pos_lod);
                }
            }
        }
        self.chunk_loader.request(&request_list)?;
        self.last_chunk_load = player_chunk_pos;
        trace!("Requested chunks: {}", request_list.len());
        Ok(())
    }

    pub fn unload_distant_chunks(&mut self) {
        let mut unload_list = Vec::new();
        let cam_pos = get_chunk_pos(self.center);
        for chunk_pos in self.chunks.keys() {
            let vec = [cam_pos[0] - chunk_pos[0], cam_pos[1] - chunk_pos[1]];
            let distance = f32::sqrt((vec[0] * vec[0] + vec[1] * vec[1]) as f32).round() as i32;
            if distance >= self.active_chunk_radius {
                unload_list.push(*chunk_pos);
            }
        }
        trace!("Unloading {} chunks", unload_list.len());
        for pos in unload_list {
            self.chunks.remove(&pos);
        }
    }

    pub fn get_finished_chunks(&mut self) -> Result<(), WorldError> {
        let finished_chunks = self.chunk_loader.get()?;
        if finished_chunks.len() > 0 {
            trace!("Finished chunks: {}", finished_chunks.len());
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
        if abs_pos[0] < 0
            || abs_pos[1] < 0
            || abs_pos[0] > self.size[0]
            || abs_pos[1] > self.size[1]
        {
            None
        } else {
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

    fn update_shader_resources(&self) -> Result<(), GraphicsError> {
        self.surface_shader_program.use_program();
        self.surface_shader_program
            .set_resource_vec3("view_pos", &self.center.as_glm())?;
        self.surface_shader_program
            .set_resource_vec3("light_pos", &self.sun.calculate_position().as_glm())?;

        let light_level = self.sun.calculate_light_level();
        let fog_color = Point3f::from_scalar(1. - (-light_level).exp());
        self.surface_shader_program
            .set_resource_vec3("fog_color", &fog_color.as_glm())?;
        self.skybox.update_light_level(light_level)?;
        self.surface_shader_program.use_program();
        Ok(())
    }

    fn get_chunk_by_world_pos(&self, world_pos: Point3f) -> Option<&Chunk> {
        self.chunks.get(&get_chunk_pos(world_pos))
    }
}

impl Renderable for World {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        self.surface_texture.activate();
        info.push_shader(self.surface_shader_program.clone());

        self.test_monkey.render(info)?;
        self.chunks.values().try_for_each(|c| c.render(info))?;

        info.pop_shader();

        self.surface_texture.deactivate();
        self.skybox.render(info)?;
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        if self.chunk_update_timer.fires() {
            if let Err(e) = self.get_finished_chunks() {
                error!("{}", e); // TODO: handle error
            }
            let cam_chunk_pos = get_chunk_pos(self.center);
            let vec = [
                cam_chunk_pos[0] - self.last_chunk_load[0],
                cam_chunk_pos[1] - self.last_chunk_load[1],
            ];
            if f32::sqrt((vec[0] * vec[0] + vec[1] * vec[1]) as f32) > 2. {
                self.unload_distant_chunks();
                if let Err(e) = self.request_chunks() {
                    error!("{}", e); // TODO: handle error
                }
            }
        }
        if self.chunk_build_stats_timer.fires() {
            info!(
                "Avg chunk build time = {:.2} ms, total chunk vertices = {}",
                self.chunk_loader.get_avg_build_time(),
                format_number(self.count_loaded_vertices())
            );
        }

        self.skybox.set_translation(self.center);
        self.sun.set_rotation_center(self.center);
        self.sun.tick(time_passed)?;

        self.test_monkey.mod_rotation(Point3f::new(0., 0., 0.25));
        self.test_monkey.mod_translation(Point3f::new(4., 0., 0.));
        if self.test_monkey.get_translation()[0] >= 500. {
            self.test_monkey
                .mod_translation(Point3f::new(-500., 0., 0.));
        }

        if let Err(e) = self.update_shader_resources() {
            error!("{}", e); // TODO: handle error
        }
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
        .add_resource("texture_array")
        .add_resource("mvp")
        .add_resource("model")
        .add_resource("view_pos")
        .add_resource("light_pos")
        .add_resource("fog_color")
        .finish()?;
    // setting texture slot to 0
    if let Err(e) = surface_shader_program.set_resource_integer("texture_array", 0) {
        return Err(GraphicsError::from(e).into());
    }
    Ok(surface_shader_program)
}
