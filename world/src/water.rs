use std::path::Path;
use std::rc::Rc;

use core::graphics::{
    GraphicsError, Mesh, Model, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder,
};
use core::light::SceneLights;
use core::traits::{RenderInfo, Renderable, Scalable, Translatable, Updatable};
use core::{Config, CoreError, FileError, Point2i, Point3f, UpdateError};

pub struct Water {
    shader: Rc<ShaderProgram>,
    normal_map: Texture,
    dudv_map: Texture,
    model: Model,
    mesh: Mesh,
    water_level: f32,
    dudv_offset: f32,
    dudv_offset_per_second: f32,
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
            .add_resource("normal_map")
            .add_resource("dudv_map")
            .add_resource("mvp")
            .add_resource("model")
            .add_resource("dudv_offset")
            .add_resource("view_pos")
            .add_resource("active_lights")
            .add_resource("scene_lights[0].color")
            .add_resource("scene_lights[0].world_pos")
            .add_resource("scene_lights[0].absolute_intensity")
            .add_resource("scene_lights[0].ambient_intensity")
            .add_resource("scene_lights[0].diffuse_intensity")
            .add_resource("scene_lights[0].specular_intensity")
            .add_resource("scene_lights[0].specular_shininess")
            .add_resource("scene_lights[1].color")
            .add_resource("scene_lights[1].world_pos")
            .add_resource("scene_lights[1].absolute_intensity")
            .add_resource("scene_lights[1].ambient_intensity")
            .add_resource("scene_lights[1].diffuse_intensity")
            .add_resource("scene_lights[1].specular_intensity")
            .add_resource("scene_lights[1].specular_shininess")
            .finish()?;

        shader.use_program();

        if let Err(e) = shader.set_resource_integer("normal_map", 0) {
            return Err(GraphicsError::from(e).into());
        }

        if let Err(e) = shader.set_resource_integer("dudv_map", 1) {
            return Err(GraphicsError::from(e).into());
        }

        let mesh_path = Path::new(config.get_str("water_surface_mesh")?)
            .to_str()
            .ok_or(FileError::InvalidPath("water_surface_mesh".to_owned()))?
            .to_owned();

        let normal_map_path = Path::new(config.get_str("water_normal_map")?)
            .to_str()
            .ok_or(FileError::InvalidPath("water_normal_map".to_owned()))?
            .to_owned();

        let normal_map = TextureBuilder::new_2d(Point2i::from_scalar(1024))
            //.use_mipmaps()
            .finish()?;
        normal_map.fill_with_image(&normal_map_path)?;

        let dudv_map_path = Path::new(config.get_str("water_dudv_map")?)
            .to_str()
            .ok_or(FileError::InvalidPath("water_dudv_map".to_owned()))?
            .to_owned();

        let dudv_map = TextureBuilder::new_2d(Point2i::from_scalar(512))
            //.use_mipmaps()
            .finish()?;
        dudv_map.fill_with_image(&dudv_map_path)?;

        let mut model = Model::default();
        model.set_scale(Point3f::new(1024., 1024., 1.));
        model.set_translation(Point3f::new(0., 0., 80.));

        let mesh = match Mesh::from_obj(&mesh_path) {
            Ok(m) => m,
            Err(e) => return Err(CoreError::from(GraphicsError::from(e))),
        };

        Ok(Self {
            shader: Rc::new(shader),
            normal_map: normal_map,
            dudv_map: dudv_map,
            model: model,
            mesh: mesh,
            water_level: 0.,
            dudv_offset: 0.,
            dudv_offset_per_second: 5e-3,
        })
    }

    pub fn update_shader_resources(
        &self,
        view_pos: Point3f,
        lights: &SceneLights,
    ) -> Result<(), GraphicsError> {
        self.shader.use_program();
        lights.update_lights_for_shader(&self.shader)?;
        self.shader
            .set_resource_vec3("view_pos", &view_pos.as_glm())?;
        self.shader
            .set_resource_float("dudv_offset", self.dudv_offset)?;
        Ok(())
    }

    pub fn get_shader(&self) -> &ShaderProgram {
        &self.shader
    }
}

impl Updatable for Water {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        self.dudv_offset += self.dudv_offset_per_second * (time_passed as f32 / 1000.);
        Ok(())
    }
}

impl Renderable for Water {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        info.push_shader(self.shader.clone());
        self.normal_map.activate(0);
        self.dudv_map.activate(1);

        let mvp = info.get_camera().create_mvp_matrix(&self.model);
        info.get_active_shader().set_resource_mat4("mvp", &mvp)?;
        info.get_active_shader()
            .set_resource_mat4("model", self.model.get_matrix_ref())?;
        self.mesh.render(info)?;

        self.dudv_map.deactivate();
        self.normal_map.deactivate();
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
