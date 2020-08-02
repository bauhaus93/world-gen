use std::path::Path;
use std::rc::Rc;

use core::file::read_image;
use core::graphics::{
    GraphicsError, Mesh, Model, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder,
};
use core::{Config, UpdateError, CoreError, FileError, Point2i, Point3f, Point3i};
use core::traits::{Scalable, RenderInfo, Renderable, Translatable, Updatable};

pub struct Water {
    shader: Rc<ShaderProgram>,
    model: Model,
    mesh: Mesh,
    water_level: f32
}

impl Water {
    pub fn new(config: &Config) -> Result<Self, CoreError> {
        let shader_dir = Path::new(config.get_str("water_shader_directory")?);

        let vertex_shader_path = shader_dir
            .join("VertexShader.glsl")
            .to_str()
            .ok_or(FileError::InvalidPath("water_vertex_shader".to_owned()))?
            .to_owned();
        let fragment_shader_path = shader_dir
            .join("FragmentShader.glsl")
            .to_str()
            .ok_or(FileError::InvalidPath("water_fragment_shader".to_owned()))?
            .to_owned();

        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader(&vertex_shader_path)
            .add_fragment_shader(&fragment_shader_path)
            .add_resource("mvp")
            .finish()?;

        let mesh_path = Path::new(config.get_str("water_surface_mesh")?)
            .to_str()
            .ok_or(FileError::InvalidPath("water_surface_mesh".to_owned()))?
            .to_owned();

        let mut model = Model::default();
        model.set_scale(Point3f::new(1000., 1000., 1.));
        model.set_translation(Point3f::new(0., 0., 80.));

        let mesh = match Mesh::from_obj(&mesh_path) {
            Ok(m) => m,
            Err(e) => return Err(CoreError::from(GraphicsError::from(e))),
        };

        Ok(Self {
            shader: Rc::new(shader),
            model: model,
            mesh: mesh,
            water_level: 0.
        })
    }
}

impl Updatable for Water {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {

        Ok(())
    }
}

impl Renderable for Water {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        info.push_shader(self.shader.clone());

        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        info.get_active_shader().set_resource_mat4("mvp", &mvp)?;
        self.mesh.render(info)?;

        info.pop_shader();
        Ok(())
    }
}

impl Translatable for Water {
    fn set_translation(&mut self, mut new_translation: Point3f) {
        new_translation[2] = self.water_level;
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Point3f {
        self.model.get_translation()
    }
}
