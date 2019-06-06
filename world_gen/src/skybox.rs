
use graphics::{ ShaderProgram, ShaderProgramBuilder, texture::Texture };
use crate::object::Object;
use crate::WorldError;

pub struct Skybox {
    shader: ShaderProgram,
    texture: Texture,
    box_data: Object
}

impl Skybox {
    pub fn new(img_file: &str) -> Result<Self, WorldError> {
        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/skybox/VertexShader.glsl")
            .add_fragment_shader("resources/shader/skybox/FragmentShader.glsl")
            .add_resource("mvp")
            .add_resource("texture")
            .finish()?;
        let texture = Texture::new(img_file)?;
        Ok(sb)
    }
}