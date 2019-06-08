use glm::{ Vector3 };

use utility::Float;
use graphics::{ GraphicsError, ShaderProgram, Mesh };
use crate::{ camera::Camera, Model };
use crate::traits::{ Rotatable, Translatable, Scalable, Renderable };

pub struct ObjectLOD {
    model: Model,
    mesh_list: [Mesh; 2]
}

impl ObjectLOD {
    pub fn new(mesh_list: [Mesh; 2]) -> ObjectLOD {
        ObjectLOD {
            model: Model::default(),
            mesh_list: mesh_list
        }
    }
}

impl Renderable for ObjectLOD {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError> {
        debug_assert!(lod < 2);
        let mvp = camera.create_mvp_matrix(&self.model);
        shader.set_resource_mat4("mvp", &mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.mesh_list[lod as usize].render()?;
        Ok(()) 
    }
}

impl Translatable for ObjectLOD {
    fn set_translation(&mut self, new_translation: Vector3<Float>) {
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.model.get_translation()
    }
}

impl Rotatable for ObjectLOD {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>) {
        self.model.set_rotation(new_rotation);
    }
    fn get_rotation(&self) -> Vector3<Float> {
        self.model.get_rotation()
    }
}

impl Scalable for ObjectLOD {
    fn set_scale(&mut self, new_scale: Vector3<Float>) {
        self.model.set_scale(new_scale);
    }
    fn get_scale(&self) -> Vector3<Float> {
        self.model.get_scale()
    }
}