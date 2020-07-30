use glm::Vector3;

use super::{ChunkError, CHUNK_SIZE};
use crate::{architect::Architect, HeightMap};
use core::graphics::{GraphicsError, Mesh};
use core::traits::{RenderInfo, Renderable, Translatable};
use core::{BoundingBox, Model, ObjectManager, Point2f, Point2i};

pub struct Chunk {
    pos: Point2i,
    model: Model,
    mesh: Mesh,
    height_map: HeightMap,
    lod: u8,
    bounding_box: BoundingBox,
    objects: Vec<u32>,
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
            bounding_box: bounding_box,
            objects: Vec::new(),
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

    pub fn get_objects(&self) -> &[u32] {
        self.objects.as_slice()
    }

    pub fn load_objects(
        &mut self,
        object_manager: &mut ObjectManager,
        architect: &Architect,
    ) -> Result<(), ChunkError> {
        self.load_trees(object_manager, architect)?;

        Ok(())
    }

    fn load_trees(
        &mut self,
        object_manager: &mut ObjectManager,
        architect: &Architect,
    ) -> Result<(), ChunkError> {
        let tree_positions = architect.get_trees(self.pos);
        for tp in tree_positions.into_iter() {
            let id = object_manager.create_object("tree", false)?;
            object_manager.mod_object(id, |t| {
                t.set_translation(tp);
            });
            self.objects.push(id);
        }
        Ok(())
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
        }
        Ok(())
    }
}

fn build_bounding_box(height_map: &HeightMap) -> BoundingBox {
    let max_xy = (height_map.get_size() - 1) as f32 * height_map.get_resolution();
    let min = Vector3::new(0., 0., height_map.get_min() as f32);
    let max = Vector3::new(max_xy, max_xy, height_map.get_max() as f32);
    BoundingBox::from_min_max(min, max)
}
