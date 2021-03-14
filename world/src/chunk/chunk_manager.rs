use std::collections::BTreeMap;
use std::convert::TryInto;
use std::iter;
use std::rc::Rc;
use std::sync::Arc;

use super::{get_chunk_pos, get_relative_pos, Chunk, ChunkError, ChunkLoader, CHUNK_SIZE};
use crate::{Architect, HeightMap};
use core::light::SceneLights;
use core::{
    Config, GraphicsError, Mesh, Point2i, Point3f, RenderInfo, Renderable, ShaderProgram,
    ShaderProgramBuilder, Timer, Updatable, UpdateError,
};

pub struct ChunkManager {
    shader: Rc<ShaderProgram>,
    chunk_loader: ChunkLoader,
    chunk_map: BTreeMap<Point2i, Chunk>,
    build_stats_timer: Timer,
    chunk_retrieval_timer: Timer,
    mesh: [Mesh; 3],
    lod_distances: [i32; 3],
}

impl ChunkManager {
    pub fn new(architect: Architect, config: &Config) -> Result<Self, ChunkError> {
        let mesh_lod0: Mesh = HeightMap::new(CHUNK_SIZE, 1.).try_into()?;
        let mesh_lod1: Mesh = HeightMap::new(CHUNK_SIZE / 4, 4.2).try_into()?;
        let mesh_lod2: Mesh = HeightMap::new(CHUNK_SIZE / 8, 8.).try_into()?;

        let surface_shader_dir = config.get_str("surface_shader_dir")?;
        let surface_shader_program = load_surface_shader(surface_shader_dir)?;

        let mut cm = Self {
            shader: Rc::new(surface_shader_program),
            chunk_loader: ChunkLoader::new(Arc::new(architect)),
            chunk_map: BTreeMap::default(),
            build_stats_timer: Timer::new(5000),
            chunk_retrieval_timer: Timer::new(500),
            mesh: [mesh_lod0, mesh_lod1, mesh_lod2],
            lod_distances: get_lod_distances(config),
        };
        cm.chunk_loader.start(8);
        Ok(cm)
    }

    pub fn request(&mut self, center: Point3f) -> Result<(), ChunkError> {
        let mut request_list: Vec<Point2i> = Vec::new();
        let center_chunk = get_chunk_pos(center);

        let max_distance = self.lod_distances[2] as f32;
        for r in 0..self.lod_distances[2] {
            let rad_iter_x = (-r..r + 1)
                .zip(iter::repeat(r))
                .chain((-r..r + 1).zip(iter::repeat(-r)));
            let rad_iter_y = iter::repeat(r)
                .zip(-r..r + 1)
                .chain(iter::repeat(-r).zip(-r..r + 1));
            let offset_iter = rad_iter_x.chain(rad_iter_y).filter_map(|(x, y)| {
                let p = Point2i::new(x, y);
                if p.length() < max_distance {
                    Some(p)
                } else {
                    None
                }
            });
            for offset in offset_iter {
                let abs_pos = center_chunk + offset;
                if self.chunk_map.get(&abs_pos).is_none() && !request_list.contains(&abs_pos) {
                    request_list.push(abs_pos);
                }
            }
        }
        self.chunk_loader.request(request_list.as_slice())?;
        self.unload_distant_chunks(center_chunk)?;
        Ok(())
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

    pub fn get_height(&self, world_pos: Point3f) -> f32 {
        let chunk_pos = get_chunk_pos(world_pos);
        match self.chunk_map.get(&chunk_pos) {
            Some(chunk) => chunk.get_height(get_relative_pos(world_pos)),
            None => 0.,
        }
    }

    fn retrieve_loaded_chunks(&mut self) -> Result<(), ChunkError> {
        let new_chunks = self.chunk_loader.get(500)?;
        for (pos, chunk) in new_chunks.into_iter() {
            self.chunk_map.insert(pos, chunk);
        }
        Ok(())
    }

    fn unload_distant_chunks(&mut self, center: Point2i) -> Result<(), ChunkError> {
        let remove_list: Vec<Point2i> = self
            .chunk_map
            .keys()
            .filter_map(|k| {
                if (*k - center).length() > self.lod_distances[2] as f32 {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        if remove_list.len() > 0 {
            trace!("Unloading {} chunks", remove_list.len());
        }
        remove_list.into_iter().for_each(|k| {
            self.chunk_map.remove(&k);
        });
        Ok(())
    }
}

impl Updatable for ChunkManager {
    fn tick(&mut self, time_passed: u32) -> Result<(), UpdateError> {
        if self.build_stats_timer.fires() {
            info!(
                "Active chunks: {}, avg chunk build time: {:.2}ms",
                self.chunk_map.len(),
                self.chunk_loader.get_avg_build_time()
            );
        }

        if self.chunk_retrieval_timer.fires() {
            self.retrieve_loaded_chunks()
                .map_err(|e| UpdateError::Internal(e.to_string()))?;
        }

        self.build_stats_timer.tick(time_passed)?;
        self.chunk_retrieval_timer.tick(time_passed)?;
        Ok(())
    }
}

impl Renderable for ChunkManager {
    fn render<'a>(&self, info: &'a mut RenderInfo) -> Result<(), GraphicsError> {
        info.push_shader(self.shader.clone());

        self.shader.set_resource_integer("chunk_size", CHUNK_SIZE)?;
        for chunk in self.chunk_map.values() {
            if chunk.prepare_rendering(info)? {
                self.mesh[0].render(info)?;
            }
        }

        info.pop_shader();
        Ok(())
    }
}

fn load_surface_shader(directory: &str) -> Result<ShaderProgram, GraphicsError> {
    let surface_shader_program = ShaderProgramBuilder::new()
        .add_vertex_shader((directory.to_owned() + "/VertexShader.glsl").as_str())
        .add_fragment_shader((directory.to_owned() + "/FragmentShader.glsl").as_str())
        .add_resource("mvp")
        .add_resource("model")
        .add_resource("chunk_size")
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

impl Drop for ChunkManager {
    fn drop(&mut self) {
        self.chunk_loader.stop();
    }
}
