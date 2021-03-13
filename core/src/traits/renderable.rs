use crate::{graphics::GraphicsError, Camera, ShaderProgram};
use std::rc::Rc;

pub struct RenderInfo<'a> {
    shader_stack: Vec<Rc<ShaderProgram>>,
    camera: &'a Camera,
    lod: u8,
}

impl<'a> RenderInfo<'a> {
    pub fn new(camera: &'a Camera) -> RenderInfo<'a> {
        RenderInfo {
            shader_stack: Vec::new(),
            camera: camera,
            lod: 0,
        }
    }

    pub fn get_camera(&self) -> &'a Camera {
        self.camera
    }

    pub fn get_active_shader(&self) -> Result<&ShaderProgram, GraphicsError> {
        self.shader_stack
            .first()
            .map(|s| s.as_ref())
            .ok_or(GraphicsError::EmptyShaderStack)
    }

    pub fn get_lod(&self) -> u8 {
        self.lod
    }

    pub fn set_lod(&mut self, lod: u8) {
        self.lod = lod;
    }

    pub fn push_shader(&mut self, shader: Rc<ShaderProgram>) {
        shader.use_program();
        self.shader_stack.push(shader);
    }
    pub fn pop_shader(&mut self) {
        self.shader_stack.pop();
        if let Some(s) = self.shader_stack.first() {
            s.use_program();
        }
    }
}

pub trait Renderable {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError>;
}
