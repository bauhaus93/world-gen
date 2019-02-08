use std::collections::BTreeMap;
use std::convert::TryFrom;
use glm::Vector3;

use crate::application::ApplicationError;
use crate::graphics::{ Projection, Mesh, ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use crate::graphics::projection::{ create_default_orthographic, create_default_perspective };
use crate::graphics::transformation::create_direction;
use crate::utility::{ Float, format_number };
use crate::world::{ Model, Object, Camera, WorldError, chunk::{ Chunk, ChunkLoader } };
use crate::world::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };
use crate::world::noise::{ Noise, OctavedNoise };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    test_object: Object,
    chunk_loader: ChunkLoader,
    chunks: BTreeMap<[i32; 2], Chunk>
}

const TEXTURE_LAYER_MUD: u32 = 0;
const TEXTURE_LAYER_GRASS: u32 = 1;

const TEXTURES: [[u32; 3]; 2] = [
    [0, 0, TEXTURE_LAYER_MUD],
    [1, 0, TEXTURE_LAYER_GRASS]
];

impl World {
    pub fn new() -> Result<World, WorldError> {
        let mut builder = TextureArrayBuilder::new("resources/atlas.png", [32, 32]);
        for tex in TEXTURES.iter() {
            builder = builder.add_texture(tex);
        }
        let texture_array = builder.finish()?;

        let mut height_noise = OctavedNoise::default();
        height_noise.set_octaves(4);
        height_noise.set_scale(1e-3);
        height_noise.set_roughness(10.);
        height_noise.set_range([0., 20.]);

        let mut test_object = Object::new(Mesh::from_obj("resources/obj/test.obj")?);
        test_object.set_translation(Vector3::new(0., 0., 500.));
        test_object.set_scale(Vector3::new(5., 5., 5.));

        let chunk_loader = ChunkLoader::new(Box::new(height_noise));

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            test_object: test_object,
            chunk_loader: chunk_loader,
            chunks: BTreeMap::new()
        };

        world.load_chunks(5)?;
        info!("Loaded chunks: {}, loaded vertices: {}", world.count_loaded_chunks(), format_number(world.count_loaded_vertices()));

        Ok(world)
    }

    pub fn load_chunks(&mut self, radius: i32) -> Result<(), WorldError> {
        for y in -radius..radius {
            for x in -radius..radius {
                if f32::sqrt((x * x + y * y) as f32) < radius as f32 {
                    // load/request chunks
                }
            }
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

    pub fn get_sun_pos(&self) -> Vector3<Float> {
        self.test_object.get_translation()
    }

    pub fn count_loaded_chunks(&self) -> u32 {
        self.chunks.len() as u32
    }

    pub fn count_loaded_vertices(&self) -> u32 {
        let mut vertex_count = 0;
        self.chunks.iter().for_each(|(_, c)| vertex_count += c.get_vertex_count());
        vertex_count
    }

    pub fn render(&self, shader: &ShaderProgram) -> Result<(), WorldError> {
        self.texture_array.activate();

        self.test_object.render(&self.camera, shader)?;
        for (_pos, chunk) in self.chunks.iter() {
            chunk.render(&self.camera, shader)?;
        }

        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        self.test_object.mod_translation(Vector3::new(2., 0., 0.));
        if self.test_object.get_translation()[0] > 1000. {
            self.test_object.mod_translation(Vector3::new(-1000., 0., 0.));
        }
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));
    }
}

