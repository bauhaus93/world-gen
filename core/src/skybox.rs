use std::rc::Rc;
use glm::{ Vector3, GenNum };

use crate::graphics::{ Mesh, Model, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder, GraphicsError, texture::Orientation };
use crate::graphics::mesh::vertex_buffer::{ BUFFER_POSTION };
use crate::{ Float, Camera, UpdateError, CoreError};
use crate::traits::{ Translatable, Scalable, Renderable, RenderInfo };

pub struct Skybox {
    texture: Texture,
    shader: Rc<ShaderProgram>,
    model: Model,
    mesh: Mesh,
    origin_z: Float
}

impl Skybox {
    pub fn new(img_file: &str) -> Result<Self, CoreError> {
        const CUBE_PATH: &'static str = "resources/obj/cube_inward.obj";
        info!("Creating skybox from obj '{}' with img '{}'", CUBE_PATH, img_file);

        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/skybox/VertexShader.glsl")
            .add_fragment_shader("resources/shader/skybox/FragmentShader.glsl")
            .add_resource("cube_texture")
            .add_resource("mvp")
            .add_resource("light_level")
            .finish()?;
        if let Err(e) = shader.set_resource_integer("cube_texture", 0) {
            return Err(GraphicsError::from(e).into());
        }

        let mut builder = TextureBuilder::new_cube_map(img_file, 512);
        builder.add_cube_element([1, 0], Orientation::Top);
        builder.add_cube_element([1, 2], Orientation::Bottom);
        builder.add_cube_element([0, 1], Orientation::Left);
        builder.add_cube_element([2, 1], Orientation::Right);
        builder.add_cube_element([1, 1], Orientation::Front);
        builder.add_cube_element([3, 1], Orientation::Back);
        let texture = builder.finish()?;

        let mut model = Model::default();
        model.set_scale(Vector3::from_s(2000.));

        let mesh = match Mesh::from_obj_custom_buffers(CUBE_PATH, BUFFER_POSTION) {
			Ok(m) => m,
			Err(e) => return Err(CoreError::from(GraphicsError::from(e)))
		};

        let sb = Skybox {
            shader: Rc::new(shader),
            texture: texture,
            model: model,
            mesh: mesh,
            origin_z: 100.
        };

        Ok(sb)
    }

    pub fn scale(&mut self, scale: Float) {
        self.model.set_scale(Vector3::from_s(scale));
    }

    pub fn update_light_level(&self, light_level: f32) -> Result<(), GraphicsError> {
        self.shader.use_program();
        self.shader.set_resource_float("light_level", light_level)?;
        Ok(())
    }
}

impl Renderable for Skybox {
	fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(),GraphicsError> {
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
    fn set_translation(&mut self, mut new_translation: Vector3<Float>) {
        new_translation[2] = self.origin_z;
        self.model.set_translation(new_translation);
    }
    fn get_translation(&self) -> Vector3<Float> {
        self.model.get_translation()
    }
}
