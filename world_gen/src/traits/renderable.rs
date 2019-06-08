use graphics::{ ShaderProgram, GraphicsError };
use crate::camera::Camera;

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError>;
}