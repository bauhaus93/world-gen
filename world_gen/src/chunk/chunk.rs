use glm::Vector3;

use graphics::{ ShaderProgram, GraphicsError, Mesh };
use utility::Float;
use crate::traits::{ Translatable, Renderable };
use crate::{ Model, Camera };
use super::chunk_size::CHUNK_SIZE;

pub struct Chunk {
    pos: [i32; 2],
    model: Model,
    mesh: Mesh,
    lod: u8
}

impl Chunk {
    pub fn new(pos: [i32; 2], lod: u8, mesh: Mesh) -> Self {
        let mut model = Model::default();
        model.set_translation(Vector3::new((pos[0] * CHUNK_SIZE) as Float, (pos[1] * CHUNK_SIZE) as Float, 0.));
        Self {
            pos: pos,
            model: model,
            mesh: mesh,
            lod: lod
        }
    }

    pub fn get_pos(&self) -> [i32; 2] {
        self.pos
    }

    pub fn get_lod(&self) -> u8 {
        self.lod
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.mesh.get_vertex_count()
    }
}

impl Renderable for Chunk {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        let mvp = camera.create_mvp_matrix(&self.model);
        shader.set_resource_mat4("mvp", &mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.mesh.render()?;
        Ok(())
    }
}

