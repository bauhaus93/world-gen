use num_traits::One;

use glm::{ Vector2, Vector3, Matrix4 };

use core::{Float,  Model, Object, Camera, BoundingBox };
use core::graphics::{ ShaderProgram, GraphicsError, Mesh };
use core::traits::{ Translatable, Renderable };
use super::{ HeightMap, CHUNK_SIZE };

pub struct Chunk {
    pos: [i32; 2],
    model: Model,
    mesh: Mesh,
    height_map: HeightMap,
    mvp: Matrix4<Float>,
    lod: u8,
    tree_list: Vec<Object>,
    bounding_box: BoundingBox
}

impl Chunk {
    pub fn new(pos: [i32; 2], height_map: HeightMap, lod: u8, mesh: Mesh) -> Self {
        let mut model = Model::default();
        model.set_translation(Vector3::new((pos[0] * CHUNK_SIZE) as Float, (pos[1] * CHUNK_SIZE) as Float, 0.));
        let bounding_box = build_bounding_box(&height_map);

        Self {
            pos: pos,
            model: model,
            mesh: mesh,
            height_map: height_map,
            mvp: Matrix4::one(),
            lod: lod,
            tree_list: Vec::new(),
            bounding_box: bounding_box
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

    pub fn get_model(&self) -> &Model {
        &self.model
    }

    pub fn update_mvp(&mut self, new_mvp: Matrix4<Float>) {
        self.mvp = new_mvp;
    }

    pub fn get_height(&self, world_pos: Vector2<Float>) -> f64 {
        let chunk_pos = self.model.get_translation();
        let relative_pos = [(world_pos.x as f64) - (chunk_pos.x as f64),
                            (world_pos.y as f64) - (chunk_pos.y as f64)];
        self.height_map.get_interpolated_height(relative_pos)
    }

    pub fn add_tree(&mut self, tree_object: Object) {
        self.tree_list.push(tree_object);
    }

    pub fn is_visible(&self) -> bool {
        self.bounding_box.is_visible(self.mvp)
    }
}

impl Renderable for Chunk {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        shader.set_resource_mat4("mvp", &self.mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.mesh.render()?;
        for tree in &self.tree_list {
            tree.render(camera, shader, lod)?;
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

