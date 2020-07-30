use std::rc::Rc;
use std::path::Path;

use crate::graphics::mesh::vertex_buffer::BUFFER_POSTION;
use crate::graphics::{
    texture::Orientation, GraphicsError, Mesh, Model, ShaderProgram, ShaderProgramBuilder, Texture,
    TextureBuilder,
};
use crate::traits::{RenderInfo, Renderable, Scalable, Translatable};
use crate::{CoreError, FileError, Point3f, Config};

pub struct Skybox {
    texture: Texture,
    shader: Rc<ShaderProgram>,
    model: Model,
    mesh: Mesh,
    origin_z: f32,
}

impl Skybox {
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        const CUBE_PATH: &'static str = "resources/obj/cube_inward.obj";
        let cube_path = config.get_str("skybox_cube_path")?;
        let cube_img = config.get_str("skybox_texture_path")?;
        let shader_dir = Path::new(config.get_str("skybox_shader_directory")?);
        info!(
            "Creating skybox from obj '{}' with img '{}'",
            cube_path, cube_img
        );

        let vertex_shader_path = shader_dir.join("VertexShader.glsl").to_str().ok_or(FileError::InvalidPath("skybox_vertex_shader".to_owned()))?.to_owned();
        let fragment_shader_path = shader_dir.join("FragmentShader.glsl").to_str().ok_or(FileError::InvalidPath("skybox_fragment_shader".to_owned()))?.to_owned();

        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader(&vertex_shader_path)
            .add_fragment_shader(&fragment_shader_path)
            .add_resource("cube_texture")
            .add_resource("mvp")
            .add_resource("light_level")
            .finish()?;
        if let Err(e) = shader.set_resource_integer("cube_texture", 0) {
            return Err(GraphicsError::from(e).into());
        }

        let mut builder = TextureBuilder::new_cube_map(cube_img, 512);
        builder.add_cube_element([1, 0], Orientation::Top);
        builder.add_cube_element([1, 2], Orientation::Bottom);
        builder.add_cube_element([0, 1], Orientation::Left);
        builder.add_cube_element([2, 1], Orientation::Right);
        builder.add_cube_element([1, 1], Orientation::Front);
        builder.add_cube_element([3, 1], Orientation::Back);
        let texture = builder.finish()?;

        let mut model = Model::default();
        model.set_scale(Point3f::from_scalar(2000.));

        let mesh = match Mesh::from_obj_custom_buffers(CUBE_PATH, BUFFER_POSTION) {
            Ok(m) => m,
            Err(e) => return Err(CoreError::from(GraphicsError::from(e))),
        };

        let sb = Skybox {
            shader: Rc::new(shader),
            texture: texture,
            model: model,
            mesh: mesh,
            origin_z: 100.,
        };

        Ok(sb)
    }

    pub fn scale(&mut self, scale: f32) {
        self.model.set_scale(Point3f::from_scalar(scale));
    }

    pub fn update_light_level(&self, light_level: f32) -> Result<(), GraphicsError> {
        self.shader.use_program();
        self.shader.set_resource_float("light_level", light_level)?;
        Ok(())
    }
}

impl Renderable for Skybox {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        self.texture.activate();
        info.push_shader(self.shader.clone());

        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        info.get_active_shader().set_resource_mat4("mvp", &mvp)?;
        self.mesh.render(info)?;

        info.pop_shader();
        self.texture.deactivate();
        Ok(())
    }
}

impl Translatable for Skybox {
    fn set_translation(&mut self, mut new_translation: Point3f) {
        new_translation[2] = self.origin_z;
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Point3f {
        self.model.get_translation()
    }
}
