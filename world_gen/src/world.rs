use std::collections::BTreeMap;

use rand;
#[allow(unused)]
use rand::{ Rng, FromEntropy, SeedableRng };
use rand::rngs::StdRng;
use glm::{ GenNum, Vector3 };

use graphics::{ Projection, Mesh, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder, GraphicsError };
use graphics::projection::{ create_default_orthographic, create_default_perspective };
use utility::Float;
use crate::{ Timer, Object, Camera, WorldError, Skybox, Sun };
use crate::chunk::{ Chunk, ChunkLoader, CHUNK_SIZE, chunk_size::get_chunk_pos };
use crate::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };

pub struct World {
    texture_array: Texture,
    camera: Camera,
    surface_shader_program: ShaderProgram,
    test_object: Object,
    skybox: Skybox,
    sun: Sun,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<[i32; 2], Chunk>,
    chunk_update_timer: Timer,
    chunk_build_stats_timer: Timer,
    lod_near_radius: i32,
    active_chunk_radius: i32,
    last_chunk_load: [i32; 2]
}

const TEXTURE_LAYER_MUD: u32 = 0;
const TEXTURE_LAYER_GRASS: u32 = 1;

const TEXTURES: [[u32; 3]; 2] = [
    [0, 0, TEXTURE_LAYER_MUD],
    [1, 0, TEXTURE_LAYER_GRASS]
];

const NEAR_RADIUS: i32 = 20;
const ACTIVE_RADIUS: i32 = 60;

impl World {
    pub fn new() -> Result<World, WorldError> {
        let surface_shader_program = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/surface/VertexShader.glsl")
            .add_fragment_shader("resources/shader/surface/FragmentShader.glsl")
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

        let mut builder = TextureBuilder::new_2d_array("resources/img/atlas.png", [32, 32]);
        for tex in TEXTURES.iter() {
            builder.add_array_element(*tex);
        }
        let texture_array = builder.finish()?;

        let mut rng = StdRng::seed_from_u64(0);//StdRng::from_entropy();

        let mut camera = Camera::default();
        camera.set_far((ACTIVE_RADIUS * CHUNK_SIZE * 8) as Float);

        let chunk_loader = ChunkLoader::from_rng(&mut rng);

        let mut test_object = Object::new(Mesh::from_obj("resources/obj/test.obj")?);
        test_object.set_translation(Vector3::new(0., 0., 500.));
        test_object.set_scale(Vector3::new(5., 5., 5.));

        let mut skybox = Skybox::new("resources/img/sky.png")?;
        skybox.scale_to_chunk_units(ACTIVE_RADIUS * 2);

        let mut sun = Sun::default();
        sun.set_day_length(3 * 60);

        let mut world = World {
            texture_array: texture_array,
            camera: camera,
            surface_shader_program: surface_shader_program,
            test_object: test_object,
            skybox: skybox,
            sun: sun,
            chunk_loader: chunk_loader,
            chunks: BTreeMap::new(),
            chunk_update_timer: Timer::new(500),
            chunk_build_stats_timer: Timer::new(5000),
            lod_near_radius: NEAR_RADIUS,
            active_chunk_radius: ACTIVE_RADIUS,
            last_chunk_load: [0, 0]
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
                        if lod != c.get_lod() {
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
        } else {
            1
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

    pub fn update(&mut self, time_passed: u32) -> Result<(), WorldError> {
        self.tick(time_passed)
    }

    pub fn render(&self) -> Result<(), WorldError> {
        self.texture_array.activate();
        self.surface_shader_program.use_program();

        self.test_object.render(&self.camera, &self.surface_shader_program, 0)?;
        for (_pos, chunk) in self.chunks.iter() {
            chunk.render(&self.camera, &self.surface_shader_program, 0)?;
        }

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

        self.test_object.mod_translation(Vector3::new(2., 0., 0.));
        if self.test_object.get_translation()[0] > 1000. {
            self.test_object.mod_translation(Vector3::new(-1000., 0., 0.));
        }
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));

        self.skybox.set_translation(self.camera.get_translation());
        self.sun.set_rotation_center(self.camera.get_translation());
        self.sun.tick(time_passed)?;
        
        self.update_shader_resources()?;
        self.chunk_update_timer.tick(time_passed)?;
        self.chunk_build_stats_timer.tick(time_passed)?;
        Ok(())
    }
}

