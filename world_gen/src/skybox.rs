use glm::{ Vector3, GenNum };

use graphics::{ Mesh, ShaderProgram, ShaderProgramBuilder, Texture, GraphicsError };
use crate::{ Object, Camera, WorldError };
use crate::traits::{ Renderable, Scalable };

pub struct Skybox {
    shader: ShaderProgram,
    texture: Texture,
    cube: Object
}

impl Skybox {
    pub fn new(img_file: &str) -> Result<Self, WorldError> {
        const CUBE_PATH: &'static str = "resources/obj/cube.obj";
        info!("Creating skybox from obj '{}' with img '{}'", CUBE_PATH, img_file);

        let shader = ShaderProgramBuilder::new()
            .add_vertex_shader("resources/shader/skybox/VertexShader.glsl")
            .add_fragment_shader("resources/shader/skybox/FragmentShader.glsl")
            .add_resource("mvp")
            .add_resource("texture_img")
            .finish()?;
        let texture = Texture::new(img_file)?;

        let mesh = Mesh::from_obj(CUBE_PATH)?;

        let mut cube = Object::new(mesh);
        cube.set_scale(Vector3::from_s(10.));

        let sb = Skybox {
            shader: shader,
            texture: texture,
            cube: cube
        };

        Ok(sb)
    }

    // caller must restore previously set shader/textures after call
    pub fn render(&self, camera: &Camera) -> Result<(), GraphicsError> {
        self.shader.use_program();
        self.texture.activate();

        self.cube.render(camera, &self.shader)?;

        self.texture.deactivate();
        Ok(())
    }
}