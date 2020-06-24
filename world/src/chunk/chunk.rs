use glm::Vector3;

use super::CHUNK_SIZE;
use crate::HeightMap;
use core::graphics::{GraphicsError, Mesh};
use core::traits::{RenderInfo, Renderable, Translatable};
use core::{BoundingBox, Float, Model, Object, Point2f, Point2i};

pub struct Chunk {
    pos: Point2i,
    model: Model,
    mesh: Mesh,
    height_map: HeightMap,
    lod: u8,
    tree_list: Vec<Object>,
    bounding_box: BoundingBox,
}

impl Chunk {
    pub fn new(pos: Point2i, height_map: HeightMap, lod: u8, mesh: Mesh) -> Self {
        let mut model = Model::default();
        model.set_translation(Point2f::from(pos * CHUNK_SIZE).extend(0.));

        let bounding_box = build_bounding_box(&height_map);

        Self {
            pos: pos,
            model: model,
            mesh: mesh,
            height_map: height_map,
            lod: lod,
            tree_list: Vec::new(),
            bounding_box: bounding_box,
        }
    }

    pub fn get_pos(&self) -> Point2i {
        self.pos
    }

    pub fn get_lod(&self) -> u8 {
        self.lod
    }

    pub fn get_vertex_count(&self) -> u32 {
        self.mesh.get_vertex_count()
    }

    pub fn get_height(&self, world_pos: Point2f) -> f32 {
        let chunk_pos = self.model.get_translation().as_xy();
        let relative_pos = world_pos - chunk_pos;
        self.height_map.get_interpolated_height(relative_pos)
    }

    pub fn add_tree(&mut self, tree_object: Object) {
        self.tree_list.push(tree_object);
    }
}

impl Renderable for Chunk {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        let shader = info.get_active_shader();
        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        if self.bounding_box.is_visible(mvp) {
            shader.set_resource_mat4("mvp", &mvp)?;
            shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
            self.mesh.render(info)?;
            if info.get_lod() == 0 {
                for tree in &self.tree_list {
                    tree.render(info)?;
                }
            }
        }
        Ok(())
    }
}

fn build_bounding_box(height_map: &HeightMap) -> BoundingBox {
    let max_xy = ((height_map.get_size() - 1) * height_map.get_resolution()) as Float;
    let min = Vector3::new(0., 0., height_map.get_min() as Float);
    let max = Vector3::new(max_xy, max_xy, height_map.get_max() as Float);
    BoundingBox::from_min_max(min, max)
}
