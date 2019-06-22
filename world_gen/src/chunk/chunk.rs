use num_traits::One;

use glm::{ Vector2, Vector3, Vector4, Matrix4 };

use graphics::{ ShaderProgram, GraphicsError, Mesh };
use utility::Float;
use crate::traits::{ Translatable, Renderable };
use crate::{ Model, Object, Camera, BoundingBox };
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
        let bounding_box = (&height_map).into();
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

    pub fn get_height(&self, world_pos: Vector2<Float>) -> Float {
        let chunk_pos = self.model.get_translation();
        let relative_pos = [world_pos.x - chunk_pos.x,
                            world_pos.y - chunk_pos.y];
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

