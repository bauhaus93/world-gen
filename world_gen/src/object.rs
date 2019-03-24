use glm::{ Vector3 };

use graphics::{ Mesh, ShaderProgram, GraphicsError };
use utility::Float;
use crate::{ camera::Camera, model::Model };
use crate::traits::{ Rotatable, Translatable, Scalable, Renderable };


pub struct Object {
    model: Model,
    mesh: Mesh,
}

impl Object {
    pub fn new(mesh: Mesh) -> Object {
        Object {
            model: Model::default(),
            mesh: mesh,
        }
    }

    #[allow(dead_code)]
    pub fn get_vertex_count(&self) -> u32 {
        self.mesh.get_vertex_count()
    }
}

impl Renderable for Object {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError> {
        let mvp = camera.create_mvp_matrix(&self.model);
        shader.set_resource_mat4("mvp", &mvp)?;
        shader.set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.mesh.render()?;
        Ok(()) 
    }
}

impl Translatable for Object {
    fn set_translation(&mut self, new_translation: Vector3<Float>) {
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.model.get_translation()
    }
}

impl Rotatable for Object {
    fn set_rotation(&mut self, new_rotation: Vector3<Float>) {
        self.model.set_rotation(new_rotation);
    }
    fn get_rotation(&self) -> Vector3<Float> {
        self.model.get_rotation()
    }
}

impl Scalable for Object {
    fn set_scale(&mut self, new_scale: Vector3<Float>) {
        self.model.set_scale(new_scale);
    }
    fn get_scale(&self) -> Vector3<Float> {
        self.model.get_scale()
    }
}