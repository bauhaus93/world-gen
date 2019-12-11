use crate::{Camera, ShaderProgram, graphics::GraphicsError };

pub trait Renderable {
    fn render(&self, camera: &Camera, shader: &ShaderProgram, lod: u8) -> Result<(), GraphicsError>;
}
