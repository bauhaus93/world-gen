use std::convert::TryInto;

use super::{get_world_pos, ChunkError, CHUNK_SIZE};
use crate::{architect::Architect, HeightMap, Noise};
use core::graphics::{GraphicsError, Mesh, ShaderProgram};
use core::traits::{RenderInfo, Renderable, Translatable};
use core::{BoundingBox, Model, ObjectManager, Point2f, Point2i, Texture};

pub struct Chunk {
    pos: Point2i,
    model: Model,
    heightmap_texture: Texture,
}

impl Chunk {
    pub fn new(pos: Point2i, noise: &dyn Noise) -> Result<Self, ChunkError> {
        let mut model = Model::default();
        let abs_pos = get_world_pos(pos, Point2f::from_scalar(0.));
        model.set_translation(abs_pos.extend(0.));

        let texture: Texture = HeightMap::from_noise(abs_pos, CHUNK_SIZE, 1., noise).try_into()?;

        Ok(Self {
            pos: pos,
            model: model,
            heightmap_texture: texture,
        })
    }

    pub fn get_pos(&self) -> Point2i {
        self.pos
    }

    pub fn get_height(&self, world_pos: Point2f) -> f32 {
        let chunk_pos = self.model.get_translation().as_xy();
        let relative_pos = world_pos - chunk_pos;
        0. // TODO: PLACEHOLDER!
    }

    pub fn prepare_rendering(&self, info: &RenderInfo) -> Result<(), GraphicsError> {
        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        let shader = info.get_active_shader()?;
        shader.set_resource_mat4("mvp", &mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.heightmap_texture.activate(0);
        Ok(())
    }
}
