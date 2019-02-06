use std::collections::BTreeSet;
use glm::Vector3;

use crate::application::ApplicationError;
use crate::graphics::{ Projection, Mesh, ShaderProgram, TextureArray, TextureArrayBuilder, GraphicsError };
use crate::graphics::projection::{ create_default_orthographic, create_default_perspective };
use crate::graphics::transformation::create_direction;
use crate::world::{ Object, Camera, WorldError };
use crate::world::traits::{ Translatable, Rotatable, Scalable, Updatable, Renderable };
use crate::world::noise::{ Noise, OctavedNoise };

use crate::utility::Float;

pub struct World {
    texture_array: TextureArray,
    camera: Camera,
    test_object: Object
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
        height_noise.set_range((0., 5.));

        let mut test_object = Object::new(Mesh::from_obj("resources/obj/test.obj")?);
        test_object.set_translation(Vector3::new(-1., -1., 1.));

        let camera = Camera::default();
        let cam_dir = create_direction(camera.get_rotation());
        info!("Camera direction = {:.2}/{:.2}/{:.2}", cam_dir.x, cam_dir.y, cam_dir.z);

        let mut world = World {
            texture_array: texture_array,
            camera: camera,
            test_object: test_object
        };

        Ok(world)
    }

    pub fn move_camera(&mut self, mut offset: Vector3<Float>) {
        let curr_height = self.camera.get_translation().z;
        match self.camera.get_projection() {
            Projection::Orthographic { .. } if curr_height > 0. => { offset.z = -curr_height; },
            Projection::Orthographic { .. } if curr_height + offset.z > 0. => { offset.z = 0.; },
            _ => {}
        }
        self.camera.mod_translation(offset);
    }

    pub fn toggle_camera_projection(&mut self) {
        match self.camera.get_projection() {
            Projection::Orthographic { .. } => {
                self.camera.set_projection(create_default_perspective());
                self.camera.set_translation(Vector3::new(-10., -10., 20.));
            },
            Projection::Perspective { .. } => {
                self.camera.set_projection(create_default_orthographic());
                self.camera.set_translation(Vector3::new(0., 0., 0.));
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

        self.texture_array.deactivate();
        Ok(())
    }
}

impl Updatable for World {
    fn tick(&mut self, time_passed: u32) {
        self.test_object.mod_rotation(Vector3::new(0., 0., 5f32.to_radians()));
    }
}

