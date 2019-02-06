use crate::graphics::{ ShaderProgram, GraphicsError };
use crate::world::Camera;

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram) -> Result<(), GraphicsError>;
}
