use graphics::{ ShaderProgram, GraphicsError };
use crate::camera::Camera;

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError>;
}