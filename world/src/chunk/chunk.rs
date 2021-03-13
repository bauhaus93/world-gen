use std::convert::TryInto;

use super::{get_world_pos, ChunkError};
use crate::HeightMap;
use core::graphics::GraphicsError;
use core::traits::{RenderInfo, Translatable};
use core::{BoundingBox, Model, Point2f, Point2i, Point3f, Texture};

pub struct Chunk {
    pos: Point2i,
    model: Model,
    heightmap: HeightMap,
    height_normal_texture: Texture,
    bounding_box: BoundingBox,
}

impl Chunk {
    pub fn new(pos: Point2i, heightmap: HeightMap) -> Result<Self, ChunkError> {
        let mut model = Model::default();
        let abs_pos = get_world_pos(pos, Point2f::from_scalar(0.));
        model.set_translation(abs_pos.extend(0.));

        let texture: Texture = heightmap.clone().try_into()?;
        let bounding_box = build_bounding_box(&heightmap);

        Ok(Self {
            pos: pos,
            model: model,
            heightmap: heightmap,
            height_normal_texture: texture,
            bounding_box: bounding_box,
        })
    }

    pub fn get_pos(&self) -> Point2i {
        self.pos
    }

    pub fn get_height(&self, relative_pos: Point2f) -> f32 {
        self.heightmap.get_interpolated_height(relative_pos)
    }

    pub fn prepare_rendering(&self, info: &RenderInfo) -> Result<bool, GraphicsError> {
        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        if self.bounding_box.is_visible(mvp) {
            let shader = info.get_active_shader()?;
            shader.set_resource_mat4("mvp", &mvp)?;
            shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
            self.height_normal_texture.activate(0);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn build_bounding_box(height_map: &HeightMap) -> BoundingBox {
    let max_xy = (height_map.get_size() - 1) as f32;
    let min = Point3f::new(0., 0., height_map.get_min() as f32);
    let max = Point3f::new(max_xy, max_xy, height_map.get_max() as f32);
    BoundingBox::from_min_max(min, max)
}
