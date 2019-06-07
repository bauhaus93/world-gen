use glm::{ Vector3, GenNum };

use utility::Float;
use graphics::{ Mesh, ShaderProgram, ShaderProgramBuilder, Texture, TextureBuilder, GraphicsError };
use crate::{ Model, Camera, WorldError };
use crate::traits::{ Translatable, Scalable };

pub struct Skybox {
    texture: Texture,
    shader: ShaderProgram,
    model: Model,
    mesh: Mesh,
    origin_z: Float
}

impl Skybox {
    pub fn new(img_file: &str) -> Result<Self, WorldError> {
        const CUBE_PATH: &'static str = "resources/obj/cube_inward.obj";
        info!("Creating skybox from obj '{}' with img '{}'", CUBE_PATH, img_file);

        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/skybox/VertexShader.glsl")
            .add_fragment_shader("resources/shader/skybox/FragmentShader.glsl")
            .add_resource("texture_img")
            .add_resource("mvp")
            .finish()?;
        if let Err(e) = shader.set_resource_integer("texture_img", 0) {
            return Err(GraphicsError::from(e).into());
        }

        let texture = TextureBuilder::new_2d(img_file).finish()?;

        let mut model = Model::default();
        model.set_scale(Vector3::from_s(750.));

        let mesh = Mesh::from_obj(CUBE_PATH)?;

        let sb = Skybox {
            shader: shader,
            texture: texture,
            model: model,
            mesh: mesh,
            origin_z: 100.
        };

        Ok(sb)
    }

    // caller must restore previously set shader/textures after call
    pub fn render(&self, camera: &Camera) -> Result<(), GraphicsError> {
        self.texture.activate();
        self.shader.use_program();

        let mvp = camera.create_mvp_matrix(&self.model);
        self.shader.set_resource_mat4("mvp", &mvp)?;
        self.mesh.render()?;

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