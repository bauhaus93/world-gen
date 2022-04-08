use std::collections::{BTreeMap, BTreeSet};

use super::Light;
use crate::graphics::{GraphicsError, ShaderProgram};

const MAX_SCENE_LIGHTS: u8 = 8;

pub struct SceneLights {
    light_map: BTreeMap<String, (u8, Light)>,
}

impl SceneLights {
    pub fn add_light(&mut self, light_name: &str, light: Light) -> bool {
        info!("Adding light to scene lights: '{}'", light_name);
        if self.light_map.len() < (MAX_SCENE_LIGHTS as usize)
            && !self.light_map.contains_key(light_name)
        {
            let index = match self.get_next_index() {
                Some(i) => i,
                None => return false,
            };
            self.light_map.insert(light_name.to_owned(), (index, light));
            true
        } else {
            false
        }
    }

    pub fn get_light_mut(&mut self, light_name: &str) -> Option<&mut Light> {
        self.light_map.get_mut(light_name).map(|(_, l)| l)
    }

    fn get_next_index(&self) -> Option<u8> {
        let mut used_indices = BTreeSet::new();
        for (_, (i, _)) in &self.light_map {
            used_indices.insert(*i);
        }
        let mut all_indices = BTreeSet::new();
        (0..MAX_SCENE_LIGHTS).for_each(|e| {
            all_indices.insert(e);
        });
        let free_indices = all_indices.difference(&used_indices);
        let min_index = free_indices
            .into_iter()
            .fold(MAX_SCENE_LIGHTS, |m, i| u8::min(m, *i));
        Some(min_index)
    }

    pub fn update_lights_for_shader(
        &self,
        shader_program: &ShaderProgram,
    ) -> Result<(), GraphicsError> {
        shader_program.set_resource_integer("active_lights", self.light_map.len() as i32)?;
        for (index, light) in self.light_map.values() {
            shader_program.set_resource_vec3(
                &format!("scene_lights[{}].color", index),
                &light.get_color().as_glm(),
            )?;
            shader_program.set_resource_vec3(
                &format!("scene_lights[{}].world_pos", index),
                &light.get_world_pos().as_glm(),
            )?;
            shader_program.set_resource_float(
                &format!("scene_lights[{}].absolute_intensity", index),
                light.get_absolute_intensity(),
            )?;
            shader_program.set_resource_float(
                &format!("scene_lights[{}].ambient_intensity", index),
                light.get_ambient_intensity(),
            )?;
            shader_program.set_resource_float(
                &format!("scene_lights[{}].diffuse_intensity", index),
                light.get_diffuse_intensity(),
            )?;
            shader_program.set_resource_float(
                &format!("scene_lights[{}].specular_intensity", index),
                light.get_specular_intensity(),
            )?;
            shader_program.set_resource_float(
                &format!("scene_lights[{}].specular_shininess", index),
                light.get_specular_shininess(),
            )?;
        }
        Ok(())
    }
}

impl Default for SceneLights {
    fn default() -> Self {
        Self {
            light_map: BTreeMap::new(),
        }
    }
}
