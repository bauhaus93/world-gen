use std::collections::BTreeSet;
use std::convert::TryFrom;
use glm::Vector3;

use crate::application::ApplicationError;
use crate::graphics::{ Projection, Mesh, ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use crate::graphics::projection::{ create_default_orthographic, create_default_perspective };
use crate::graphics::transformation::create_direction;
use crate::utility::Float;
use crate::world::{ Object, Camera, WorldError, Chunk, chunk::create_chunk_vertices };
use crate::world::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };
use crate::world::noise::{ Noise, OctavedNoise };

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    test_object: Object,
    chunk: Chunk
}

const TEXTURE_LAYER_MUD: i32 = 0;

const TEXTURES: [[i32; 3]; 1] = [
    [0, 0, TEXTURE_LAYER_MUD]
];

impl World {
    pub fn new(top_level: i32, layer_size: [i32; 2]) -> Result<World, WorldError> {
        debug_assert!(top_level > 0);
        debug_assert!(layer_size[0] > 0 && layer_size[1] > 0);
        let texture_array = TextureArrayBuilder::new("resources/atlas.png", [32, 32])
            .add_texture([0, 0, 0])
            .finish()?;

        let mut height_noise = OctavedNoise::default();
        height_noise.set_octaves(4);
        height_noise.set_scale(8e-3);
        height_noise.set_roughness(1e+3);
        height_noise.set_range([0., 5.]);

        let mut test_object = Object::new(Mesh::from_obj("resources/obj/test.obj")?);
        test_object.set_translation(Vector3::new(0., 0., 0.));

        let chunk_mesh = Mesh::try_from(create_chunk_vertices([0, 0], &height_noise))?;
        let chunk = Chunk::new([0, 0], chunk_mesh);

        let mut world = World {
            texture_array: texture_array,
            camera: Camera::default(),
            test_object: test_object,
            chunk: chunk
        };

        Ok(world)
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

    pub fn render(&self, shader: &ShaderProgram) -> Result<(), WorldError> {
        self.texture_array.activate();

        self.test_object.render(&self.camera, shader)?;
        self.chunk.render(&self.camera, shader)?;

        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));
    }
}

