use num_traits::One;

use glm::{ Vector3, Vector4, Matrix4 };

use graphics::{ ShaderProgram, GraphicsError, Mesh };
use utility::Float;
use crate::traits::{ Translatable, Renderable };
use crate::{ Model, Object, Camera };
use super::{ HeightMap, CHUNK_SIZE };

pub struct Chunk {
    pos: [i32; 2],
    model: Model,
    mesh: Mesh,
    height_map: HeightMap,
    mvp: Matrix4<Float>,
    lod: u8,
    tree_list: Vec<Object>
}

impl Chunk {
    pub fn new(pos: [i32; 2], height_map: HeightMap, lod: u8, mesh: Mesh) -> Self {
        let mut model = Model::default();
        model.set_translation(Vector3::new((pos[0] * CHUNK_SIZE) as Float, (pos[1] * CHUNK_SIZE) as Float, 0.));
        Self {
            pos: pos,
            model: model,
            mesh: mesh,
            height_map: height_map,
            mvp: Matrix4::one(),
            lod: lod,
            tree_list: Vec::new()
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

    pub fn add_tree(&mut self, tree_object: Object) {
        self.tree_list.push(tree_object);
    }

    pub fn get_clip_coordinates(&self, corner: u8) -> Vector4<Float> {
        debug_assert!(corner < 4);
        let size = self.height_map.get_size() - 1;
        let resolution = self.height_map.get_resolution();
        match corner {
            0 => self.mvp * Vector4::new(0., 0., self.height_map.get(&[0, 0]), 1.),
            1 => self.mvp * Vector4::new((size * resolution) as Float, 0., self.height_map.get(&[size, 0]), 1.),
            2 => self.mvp * Vector4::new(0., (size * resolution) as Float, self.height_map.get(&[0, size]), 1.),
            3 => self.mvp * Vector4::new((size * resolution) as Float, (size * resolution) as Float, self.height_map.get(&[size, size]), 1.), 
            _ => unreachable!()
        }
    }

    pub fn is_visible(&self) -> bool {
        for i in 0..4 {
            let clip = self.get_clip_coordinates(i);
            if clip.x.abs() < clip.w &&
               clip.y.abs() < clip.w &&
               clip.z.abs() < clip.w {
                return true;
            }
        }
        false
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

