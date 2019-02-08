
use glm::Vector3;

use crate::graphics::{ ShaderProgram, GraphicsError, mesh::{ Mesh, Vertex, Triangle, Buffer } };
use crate::world::traits::{ Translatable, Renderable };
use crate::world::{ Object, Camera, Noise };
use crate::utility::Float;
use super::CHUNK_SIZE;

pub struct Chunk {
    object: Object
}

impl Chunk {
    pub fn new(pos: [i32; 2], mesh: Mesh) -> Self {
        let mut object = Object::new(mesh);
        object.set_translation(Vector3::new((pos[0] * CHUNK_SIZE[0]) as Float, (pos[1] * CHUNK_SIZE[1]) as Float, 0.));
        Self {
            object: object
        }
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

