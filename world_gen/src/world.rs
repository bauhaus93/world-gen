use std::rc::Rc;
use std::sync::Arc;

use rand;
#[allow(unused)]
use rand::{ Rng, FromEntropy, SeedableRng };
use rand::rngs::StdRng;
use glm::{ GenNum, Vector2, Vector3, normalize, length };

use graphics::{ ShaderProgram, ShaderProgramBuilder, GraphicsError };
use utility::{ Config, Float, format_number, get_distance_2d_from_zero };
use crate::{ Player, Timer, Camera, WorldError, Skybox, Sun, ObjectManager, Object, SurfaceTexture };
use crate::chunk::{ Chunk, ChunkTree, ChunkLoader, CHUNK_SIZE, chunk_size::get_chunk_pos };
use crate::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };

pub struct World {
    surface_texture: SurfaceTexture,
    camera: Camera,
    player: Player,
    surface_shader_program: ShaderProgram,
    skybox: Skybox,
    sun: Sun,
    chunk_loader: ChunkLoader,
    chunk_tree: Box<ChunkTree>,
    chunk_update_timer: Timer,
    chunk_build_stats_timer: Timer,
    lod_near_radius: i32,
    lod_far_radius: i32,
    active_chunk_radius: i32,
    last_chunk_load: Vector2<i32>,
    #[allow(unused)]
    object_manager: Arc<ObjectManager>,
    test_monkey: Object
}

impl World {
    pub fn new(config: &Config) -> Result<World, WorldError> {

        let object_prototypes_path = config.get_str("object_prototype_path")?;
        let day_length = config.get_uint_or_default("day_length", 180);
        let skybox_img_path = config.get_str("skybox_img_path")?;
        let surface_texture_info_path = config.get_str("surface_info_path")?;

        let surface_shader_program = load_surface_shader(config)?;
        let surface_texture = SurfaceTexture::load(surface_texture_info_path)?;

        let (near_radius, far_radius, active_radius) = get_chunk_radii(config);

        info!("Day length is {}s", day_length);

        let mut rng = StdRng::seed_from_u64(0);//StdRng::from_entropy();

        let object_manager = Arc::new(ObjectManager::from_yaml(&object_prototypes_path)?);
        let chunk_loader = ChunkLoader::new(&mut rng, object_manager.clone(), surface_texture.get_terrain_set());

        let mut test_monkey = object_manager.create_object("monkey")?;
        test_monkey.set_translation(Vector3::new(0., 0., 400.));
        test_monkey.set_scale(Vector3::from_s(10.));

        let player = Player::default();
        let chunk_tree = ChunkTree::new(get_chunk_pos(player.get_translation()), active_radius);

        let mut world = World {
            surface_texture: surface_texture,
            camera: Camera::default(),
            player: Player::default(),
            surface_shader_program: surface_shader_program,
            skybox: Skybox::new(skybox_img_path)?,
            sun: Sun::with_day_length(day_length),
            chunk_loader: chunk_loader,
            chunk_tree: Box::new(chunk_tree),
            chunk_update_timer: Timer::new(500),
            chunk_build_stats_timer: Timer::new(5000),
            lod_near_radius: near_radius,
            lod_far_radius: far_radius,
            active_chunk_radius: active_radius,
            last_chunk_load: Vector2::from_s(0),
            object_manager: object_manager,
            test_monkey: test_monkey
        };

        world.update_camera_far();
        world.update_skybox_size();

        world.chunk_loader.start(8);
        world.update_chunktree()?;

        Ok(world)
    }


    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn update_chunktree(&mut self) -> Result<(), WorldError> {
        let center = get_chunk_pos(self.player.get_translation());
        let new_tree = self.chunk_tree.rebuild(center, self.active_chunk_radius);
        let missing_chunks = new_tree.get_missing_chunks(self.lod_near_radius, self.lod_far_radius);
        self.chunk_loader.request(&missing_chunks)?;

        self.chunk_tree = Box::new(new_tree);

        debug!("Requested chunks: {}", missing_chunks.len());
        Ok(())
    }


    pub fn get_finished_chunks(&mut self) -> Result<(), WorldError> {
        let finished_chunks = self.chunk_loader.get()?;
        if finished_chunks.len() > 0 {
            debug!("Finished chunks: {}", finished_chunks.len());
            finished_chunks.into_iter()
                .for_each(|chunk| self.chunk_tree.insert(Rc::new(chunk)));
        }
        Ok(())
    }

    pub fn update(&mut self, time_passed: u32) -> Result<(), WorldError> {
        self.tick(time_passed)
    }

    pub fn render(&self) -> Result<(), WorldError> {
        self.surface_texture.activate();
        self.surface_shader_program.use_program();

        self.test_monkey.render(&self.camera, &self.surface_shader_program, 0)?;

        self.chunk_tree.render(&self.camera, &self.surface_shader_program, 0)?;

        self.surface_texture.deactivate();
        self.skybox.render(&self.camera)?;
        Ok(())
    }

    fn update_camera_far(&mut self) {
        self.camera.set_far((self.active_chunk_radius * CHUNK_SIZE * 8) as Float);
    }

    fn update_skybox_size(&mut self) {
        self.skybox.scale_to_chunk_units(self.active_chunk_radius * 2);
    }

    fn update_shader_resources(&self) -> Result<(), GraphicsError> {
        self.surface_shader_program.use_program();
        self.surface_shader_program.set_resource_vec3("view_pos", &self.camera.get_translation())?;
        self.surface_shader_program.set_resource_vec3("light_pos", &self.sun.calculate_position())?;

        let light_level = self.sun.calculate_light_level();
        let fog_color = Vector3::from_s(1. - (-light_level).exp());
        self.surface_shader_program.set_resource_vec3("fog_color", &fog_color)?;
        self.skybox.update_light_level(light_level)?;
        self.surface_shader_program.use_program();
        Ok(())
    }

    fn get_chunk_by_world_pos(&self, world_pos: Vector3<Float>) -> Option<Rc<Chunk>> {
        self.chunk_tree.get_chunk(get_chunk_pos(world_pos))
    }

    fn handle_player(&mut self, time_passed: u32) -> Result<(), WorldError> {
        let player_pos = self.player.get_translation();

        let chunk_height = match self.get_chunk_by_world_pos(player_pos) {
            Some(chunk) => {
                let player_pos_xy = player_pos.truncate(2);
                let height = chunk.get_height(player_pos_xy);
                let forward_xy = normalize(self.player.get_direction().truncate(2));
                let forward_height = chunk.get_height(player_pos_xy + forward_xy);
                let forward_z = forward_height - height;

                self.player.update_forward(forward_xy.extend(forward_z as Float));
                height
            },
            None => {
                trace!("Player not on any chunk!");
                player_pos.z as f64
            }
        };
        self.player.tick(time_passed)?;

        let height_diff = self.player.get_z() as f64 - chunk_height;
        if height_diff > 0. {
            if height_diff > 0.1 && !self.player.is_jumping() {
                self.player.toggle_jump();
            }
            self.player.push_z(-0.25);
        } else {
            if self.player.is_jumping() {
                self.player.land();
            }
            self.player.move_z(-height_diff as Float);
        }

        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), WorldError> {
        if self.chunk_update_timer.fires() {
            self.get_finished_chunks()?;
            if self.chunk_tree.needs_update(get_chunk_pos(self.player.get_translation())) {
                self.update_chunktree()?;
            }
        }
        if self.chunk_build_stats_timer.fires() {
            info!("Avg chunk build time = {:.2} ms", self.chunk_loader.get_avg_build_time());
        }

        self.handle_player(time_passed)?;

        self.player.align_camera(&mut self.camera);
        self.chunk_tree.update_mvps(&self.camera);

        self.skybox.set_translation(self.player.get_translation());
        self.sun.set_rotation_center(self.player.get_translation());
        self.sun.tick(time_passed)?;

        self.test_monkey.mod_rotation(Vector3::new(0., 0., 0.25));
        self.test_monkey.mod_translation(Vector3::new(4., 0., 0.));
        if self.test_monkey.get_translation()[0] >= 500. {
            self.test_monkey.mod_translation(Vector3::new(-500., 0., 0.));
        }

        self.update_shader_resources()?;
        self.chunk_update_timer.tick(time_passed)?;
        self.chunk_build_stats_timer.tick(time_passed)?;
        Ok(())
    }
}

fn get_chunk_radii(config: &Config) -> (i32, i32, i32) {
    let active_radius = config.get_int_or_default("active_radius", 40);
    let far_radius = i32::min(config.get_int_or_default("far_radius", 3 * active_radius / 2), active_radius);
    let near_radius = i32::min(config.get_int_or_default("near_radius", active_radius / 3), far_radius);
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
