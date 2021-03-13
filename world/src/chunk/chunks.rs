use std::collections::{BTreeMap, HashSet};
use std::convert::TryInto;
use std::iter;
use std::rc::Rc;

use super::{get_chunk_pos, Chunk, ChunkError, CHUNK_SIZE};
use crate::{noise::get_default_noise, HeightMap, Noise};
use core::light::SceneLights;
use core::{
    Config, GraphicsError, Mesh, Point2i, Point3f, RenderInfo, Renderable, Seed, ShaderProgram,
    ShaderProgramBuilder, Updatable, UpdateError,
};

pub struct Chunks {
    shader: Rc<ShaderProgram>,
    chunk_map: BTreeMap<Point2i, Chunk>,
    mesh: Mesh,
    lod_distances: [i32; 3],
    height_noise: Box<dyn Noise>,
}

impl Chunks {
    pub fn new(config: &Config) -> Result<Self, ChunkError> {
        let mesh: Mesh = HeightMap::new(CHUNK_SIZE, 1.)
            .triangulate()
            .ok_or(ChunkError::HeightmapTriangulation)
            .and_then(|t| t.as_slice().try_into().map_err(ChunkError::from))?;

        let surface_shader_dir = config.get_str("surface_shader_dir")?;
        let surface_shader_program = load_surface_shader(surface_shader_dir)?;
        let noise = get_default_noise(Seed::from_entropy());

        Ok(Self {
            shader: Rc::new(surface_shader_program),
            chunk_map: BTreeMap::default(),
            mesh: mesh,
            lod_distances: get_lod_distances(config),
            height_noise: noise,
        })
    }

    pub fn request(&mut self, center: Point3f) {
        let mut request_list: Vec<Point2i> = Vec::new();
        let center_chunk_pos = get_chunk_pos(center);

        for r in 0..self.lod_distances[2] {
            let rad_iter_x = (-r..r + 1)
                .zip(iter::repeat(r))
                .chain((-r..r + 1).zip(iter::repeat(-r)));
            let rad_iter_y = iter::repeat(r)
                .zip(-r..r + 1)
                .chain(iter::repeat(-r).zip(-r..r + 1));
            for pos in rad_iter_x.chain(rad_iter_y) {
                let abs_pos = center_chunk_pos + Point2i::new(pos.0, pos.1);
                if self.chunk_map.get(&abs_pos).is_none() && !request_list.contains(&abs_pos) {
                    request_list.push(abs_pos);
                }
            }
        }
        let max_distance = self.lod_distances[2] as f32;
        for req in request_list
            .into_iter()
            .filter(|p| (*p - center_chunk_pos).get_length() < max_distance)
            .take(100)
        {
            self.chunk_map
                .insert(req, Chunk::new(req, self.height_noise.as_ref()).unwrap());
        }
    }

    pub fn update_shader_resources(
        &mut self,
        world_center: Point3f,
        fog_color: Point3f,
        scene_lights: &SceneLights,
    ) -> Result<(), GraphicsError> {
        self.shader.use_program();
        self.shader
            .set_resource_vec3("view_pos", &world_center.as_glm())?;
        self.shader
            .set_resource_vec3("fog_color", &fog_color.as_glm())?;

        scene_lights.update_lights_for_shader(&self.shader)?;
        Ok(())
    }
}

impl Updatable for Chunks {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        Ok(())
    }
}

impl Renderable for Chunks {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        info.push_shader(self.shader.clone());

        for chunk in self.chunk_map.values() {
            chunk.prepare_rendering(info)?;
            self.mesh.render(info)?;
        }

        info.pop_shader();
        Ok(())
    }
}

fn load_surface_shader(directory: &str) -> Result<ShaderProgram, GraphicsError> {
    let surface_shader_program = ShaderProgramBuilder::new()
        .add_vertex_shader((directory.to_owned() + "/VertexShader.glsl").as_str())
        .add_fragment_shader((directory.to_owned() + "/FragmentShader.glsl").as_str())
        //.add_resource("texture_array")
        .add_resource("mvp")
        .add_resource("model")
        .add_resource("view_pos")
        .add_resource("fog_color")
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
    // setting texture slot to 0
    /*if let Err(e) = surface_shader_program.set_resource_integer("texture_array", 0) {
        return Err(GraphicsError::from(e).into());
    }*/
    Ok(surface_shader_program)
}

fn get_lod_distances(config: &Config) -> [i32; 3] {
    let active_radius = config.get_int_or_default("active_radius", 40);
    let far_radius = i32::min(
        config.get_int_or_default("far_radius", 3 * active_radius / 2),
        active_radius,
    );
    let near_radius = i32::min(
        config.get_int_or_default("near_radius", active_radius / 3),
        far_radius,
    );
    [near_radius, far_radius, active_radius]
}
