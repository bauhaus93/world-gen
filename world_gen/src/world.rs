use std::collections::BTreeMap;
use std::sync::Arc;

use rand;
#[allow(unused)]
use rand::{ Rng, FromEntropy, SeedableRng };
use rand::rngs::StdRng;
use glm::{ GenNum, Vector3 };

use graphics::{ Projection, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder, GraphicsError };
use graphics::projection::{ create_default_orthographic, create_default_perspective };
use utility::{ Config, Float };
use crate::{ Timer, Camera, WorldError, Skybox, Sun, ObjectManager, Object };
use crate::chunk::{ Chunk, ChunkLoader, CHUNK_SIZE, chunk_size::get_chunk_pos };
use crate::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };

pub struct World {
    texture_array: Texture,
    camera: Camera,
    surface_shader_program: ShaderProgram,
    skybox: Skybox,
    sun: Sun,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<[i32; 2], Chunk>,
    chunk_update_timer: Timer,
    chunk_build_stats_timer: Timer,
    lod_near_radius: i32,
    lod_far_radius: i32,
    active_chunk_radius: i32,
    last_chunk_load: [i32; 2],
    object_manager: Arc<ObjectManager>,
    test_monkey: Object
}

const TEXTURE_LAYER_MUD: u32 = 0;
const TEXTURE_LAYER_GRASS: u32 = 1;

const TEXTURES: [[u32; 3]; 2] = [
    [0, 0, TEXTURE_LAYER_MUD],
    [1, 0, TEXTURE_LAYER_GRASS]
];

impl World {
    pub fn new(config: &Config) -> Result<World, WorldError> {
        let surface_shader_dir = config.get_str_or_default("surface_shader_dir", "resources/shader/surface");
        let surface_atlas_path = config.get_str_or_default("surface_atlas_path", "resources/img/atlas.png");
        let object_prototypes_path = config.get_str_or_default("object_prototype_path", "resources/prototypes.json");


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

        let mut builder = TextureBuilder::new_2d_array(&surface_atlas_path, [32, 32]);
        for tex in TEXTURES.iter() {
            builder.add_array_element(*tex);
        }
        let texture_array = builder.finish()?;

        let (near_radius, far_radius, active_radius) = get_chunk_radii(config);

        let mut rng = StdRng::seed_from_u64(0);//StdRng::from_entropy();

        let mut camera = Camera::default();
        camera.set_far((active_radius * CHUNK_SIZE * 8) as Float);

        let object_manager = Arc::new(ObjectManager::from_json(&object_prototypes_path)?);
        let chunk_loader = ChunkLoader::new(&mut rng, object_manager.clone());

        let mut skybox = Skybox::new("resources/img/sky.png")?;
        skybox.scale_to_chunk_units(active_radius * 2);

        let mut sun = Sun::default();
        sun.set_day_length(3 * 60 / 6);

        let mut test_monkey = object_manager.create_object("monkey")?;
        test_monkey.set_translation(Vector3::new(0., 0., 400.));
        test_monkey.set_scale(Vector3::from_s(10.));

        let mut world = World {
            texture_array: texture_array,
            camera: camera,
            surface_shader_program: surface_shader_program,
            skybox: skybox,
            sun: sun,
            chunk_loader: chunk_loader,
            chunks: BTreeMap::new(),
            chunk_update_timer: Timer::new(500),
            chunk_build_stats_timer: Timer::new(5000),
            lod_near_radius: near_radius,
            lod_far_radius: far_radius,
            active_chunk_radius: active_radius,
            last_chunk_load: [0, 0],
            object_manager: object_manager,
            test_monkey: test_monkey
        };

        world.camera.set_translation(Vector3::new(0., 0., 200.));

        world.chunk_loader.start(8);
        world.request_chunks()?;

        Ok(world)
    }

    pub fn get_camera_direction(&self) -> Vector3<Float> {
        self.camera.get_direction()
    }

    pub fn move_camera(&mut self, offset: Vector3<Float>) {
        self.camera.mod_translation(offset);
    }

    pub fn rotate_camera(&mut self, rotation: Vector3<Float>) {
        self.camera.mod_rotation(rotation);
    }

    pub fn request_chunks(&mut self) -> Result<(), WorldError> {
        let mut request_list: Vec<([i32; 2], u8)> = Vec::new();
        let cam_chunk_pos = get_chunk_pos(self.camera.get_translation());
        for y in -self.active_chunk_radius..self.active_chunk_radius + 1 {
            for x in -self.active_chunk_radius..self.active_chunk_radius + 1 {
                let distance = f32::sqrt((x * x + y * y) as f32).round() as i32;
                if distance < self.active_chunk_radius {
                    let lod = self.lod_by_chunk_distance(distance);
                    let chunk_pos = [cam_chunk_pos[0] + x,
                                     cam_chunk_pos[1] + y];
                    if let Some(c) = self.chunks.get(&chunk_pos) {
                        let old_lod = c.get_lod();
                        if lod != old_lod && (lod < 2 || old_lod < 2) {
                           request_list.push((chunk_pos, lod)); 
                        }
                    } else {
                        request_list.push((chunk_pos, lod));
                    }

                }
            }
        }
        self.chunk_loader.request(&request_list)?;
        self.last_chunk_load = cam_chunk_pos;
        trace!("Requested chunks: {}", request_list.len());
        Ok(())
    }

    pub fn unload_distant_chunks(&mut self) {
        let mut unload_list = Vec::new();
        let cam_pos = get_chunk_pos(self.camera.get_translation());
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

    pub fn toggle_camera_projection(&mut self) {
        match self.camera.get_projection() {
            Projection::Orthographic { .. } => {
                self.camera.set_projection(create_default_perspective());
            },
            Projection::Perspective { .. } => {
                self.camera.set_projection(create_default_orthographic());
            }
        }
    }

    #[allow(dead_code)]
    pub fn count_loaded_vertices(&self) -> u32 {
        let mut vertex_count = 0;
        self.chunks.iter().for_each(|(_, c)| vertex_count += c.get_vertex_count());
        vertex_count
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

    fn update_chunk_mvps(&mut self) {
        for c in self.chunks.values_mut() {
            c.update_mvp(self.camera.create_mvp_matrix(c.get_model()));
        }
    } 

    pub fn update(&mut self, time_passed: u32) -> Result<(), WorldError> {
        self.tick(time_passed)
    }

    pub fn render(&self) -> Result<(), WorldError> {
        self.texture_array.activate();
        self.surface_shader_program.use_program();

        self.test_monkey.render(&self.camera, &self.surface_shader_program, 0)?;
        self.chunks.values()
            .filter(|c| c.is_visible())
            .try_for_each(|c| c.render(&self.camera, &self.surface_shader_program, 0))?;

        self.texture_array.deactivate();
        self.skybox.render(&self.camera)?;
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), WorldError> {
        if self.chunk_update_timer.fires() {
            self.get_finished_chunks()?;
            let cam_chunk_pos = get_chunk_pos(self.camera.get_translation());
            let vec = [cam_chunk_pos[0] - self.last_chunk_load[0], cam_chunk_pos[1] - self.last_chunk_load[1]];
            if f32::sqrt((vec[0] * vec[0] + vec[1] * vec[1]) as f32) > 2. {
                self.unload_distant_chunks();
                self.request_chunks()?;
            }
        }
        if self.chunk_build_stats_timer.fires() {
            info!("Avg chunk build time = {:.2} ms, loaded vertices = {}", self.chunk_loader.get_avg_build_time(), self.count_loaded_vertices());
        }

        self.update_chunk_mvps();

        self.skybox.set_translation(self.camera.get_translation());
        self.sun.set_rotation_center(self.camera.get_translation());
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
    let active_radius = match config.get_int("active_radius") {
        Some(ar) => ar,
        None => 40
    };
    let far_radius = match config.get_int("far_radius") {
        Some(far) if far <= active_radius => far,
        Some(_far) => active_radius,
        None => 3 * active_radius / 2
    };
    let near_radius = match config.get_int("near_radius") {
        Some(near) if near <= far_radius => near,
        Some(_near) => far_radius,
        None => active_radius / 3
    };
    (near_radius, far_radius, active_radius)
}