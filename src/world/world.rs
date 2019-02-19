use std::collections::BTreeMap;
use glm::Vector3;

use crate::graphics::{ Projection, Mesh, ShaderProgram, ShaderProgramBuilder, TextureArray, TextureArrayBuilder, GraphicsError };
use crate::graphics::projection::{ create_default_orthographic, create_default_perspective };
use crate::world::{ Object, Camera, WorldError, chunk::{ Chunk, ChunkLoader, get_chunk_pos }, tree::Tree };
use crate::world::timer::Timer;
use crate::world::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    shader_program: ShaderProgram,
    test_object: Object,
    test_tree: Object,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<[i32; 2], Chunk>,
    chunk_update_timer: Timer,
    active_chunk_radius: i32,
    last_chunk_load: [i32; 2]
}

const TEXTURE_LAYER_MUD: u32 = 0;
const TEXTURE_LAYER_GRASS: u32 = 1;

const TEXTURES: [[u32; 3]; 2] = [
    [0, 0, TEXTURE_LAYER_MUD],
    [1, 0, TEXTURE_LAYER_GRASS]
];

impl World {
    pub fn new() -> Result<World, WorldError> {
        let shader_program = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/VertexShader.glsl")
            .add_fragment_shader("resources/shader/FragmentShader.glsl")
            .finish()?;
        shader_program.use_program();
        let mut builder = TextureArrayBuilder::new("resources/atlas.png", [32, 32]);
        for tex in TEXTURES.iter() {
            builder = builder.add_texture(tex);
        }
        let texture_array = builder.finish()?;

        let mut test_object = Object::new(Mesh::from_obj("resources/obj/test.obj")?);
        test_object.set_translation(Vector3::new(0., 0., 500.));
        test_object.set_scale(Vector3::new(5., 5., 5.));

        let tree = Tree::new();
        let mut test_tree = Object::new(tree.build_mesh()?);
        test_tree.set_scale(Vector3::new(10., 10., 10.));

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            shader_program: shader_program,
            test_object: test_object,
            test_tree: test_tree,
            chunk_loader: ChunkLoader::default(),
            chunks: BTreeMap::new(),
            chunk_update_timer: Timer::new(1000),
            active_chunk_radius: 5,
            last_chunk_load: [0, 0]
        };

        world.chunk_loader.start(2);
        world.request_chunks()?;

        Ok(world)
    }

    pub fn request_chunks(&mut self) -> Result<(), WorldError> {
        let mut request_list: Vec<[i32; 2]> = Vec::new();
        let cam_chunk_pos = get_chunk_pos(self.camera.get_translation());
        for y in -self.active_chunk_radius..self.active_chunk_radius + 1 {
            for x in -self.active_chunk_radius..self.active_chunk_radius + 1 {
                if f32::sqrt((x * x + y * y) as f32) < self.active_chunk_radius as f32 {
                    let chunk_pos = [cam_chunk_pos[0] + x,
                                     cam_chunk_pos[1] + y];
                    if !self.chunks.contains_key(&chunk_pos) {
                        request_list.push(chunk_pos);
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
        let mut unload_list = Vec::new();;
        let cam_pos = get_chunk_pos(self.camera.get_translation());
        for chunk_pos in self.chunks.keys() {
            let vec = [cam_pos[0] - chunk_pos[0], cam_pos[1] - chunk_pos[1]];
            if f32::sqrt((vec[0] * vec[0] + vec[1] * vec[1]) as f32) >= self.active_chunk_radius as f32 {
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
                self.camera.set_translation(Vector3::new(-5., -5., 5.));
            },
            Projection::Perspective { .. } => {
                self.camera.set_projection(create_default_orthographic());
                self.camera.set_translation(Vector3::new(-5., -5., 5.));
            }
        }
    }

    pub fn get_camera(&self) -> &Camera {
        &self.camera
    }

    pub fn get_camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    #[allow(dead_code)]
    pub fn count_loaded_vertices(&self) -> u32 {
        let mut vertex_count = 0;
        self.chunks.iter().for_each(|(_, c)| vertex_count += c.get_vertex_count());
        vertex_count
    }

    fn update_shader_resources(&self) -> Result<(), GraphicsError> {
        self.shader_program.set_resource_vec3("view_pos", &self.camera.get_translation())?;
        self.shader_program.set_resource_vec3("light_pos", &self.camera.get_translation())?;
        Ok(())
    }

    pub fn render(&self) -> Result<(), WorldError> {
        self.texture_array.activate();

        self.test_object.render(&self.camera, &self.shader_program)?;
        self.test_tree.render(&self.camera, &self.shader_program)?;
        for (_pos, chunk) in self.chunks.iter() {
            chunk.render(&self.camera, &self.shader_program)?;
        }

        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) -> Result<(), WorldError> {

        if self.chunk_update_timer.fires() {
            self.get_finished_chunks()?;
            let cam_chunk_pos = get_chunk_pos(self.camera.get_translation());
            let vec = [cam_chunk_pos[0] - self.last_chunk_load[0], cam_chunk_pos[1] - self.last_chunk_load[1]];
            if f32::sqrt((vec[0] * vec[0] + vec[1] * vec[1]) as f32) > self.active_chunk_radius as f32 / 10. {
                self.unload_distant_chunks();
                self.request_chunks()?;
            }
        }

        self.test_object.mod_translation(Vector3::new(2., 0., 0.));
        if self.test_object.get_translation()[0] > 1000. {
            self.test_object.mod_translation(Vector3::new(-1000., 0., 0.));
        }
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));
        self.update_shader_resources()?;
        self.chunk_update_timer.tick(time_passed)?;
        Ok(())
    }
}

