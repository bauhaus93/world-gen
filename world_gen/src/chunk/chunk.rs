use glm::Vector3;

use graphics::{ ShaderProgram, GraphicsError, mesh::Mesh };
use utility::Float;
use crate::traits::{ Translatable, Renderable };
use crate::{ object::Object, camera::Camera };
use super::chunk_size::CHUNK_SIZE;

pub struct Chunk {
    pos: [i32; 2],
    object: Object,
}

impl Chunk {
    pub fn new(pos: [i32; 2], mesh: Mesh) -> Self {
        let mut object = Object::new(mesh);
        object.set_translation(Vector3::new((pos[0] * CHUNK_SIZE[0]) as Float, (pos[1] * CHUNK_SIZE[1]) as Float, 0.));
        Self {
            pos: pos,
            object: object,
        }
    }

    pub fn get_pos(&self) -> [i32; 2] {
        self.pos
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.object.get_vertex_count()
    }
}

impl Renderable for Chunk {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        self.object.render(camera, shader)
    }
}

