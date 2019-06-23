use std::collections::BTreeMap;

use serde::Deserialize;

use graphics::{ Texture, TextureBuilder };
use utility::read_file;
use crate::WorldError;

pub struct SurfaceTexture {
    texture: Texture,
    layer_map: BTreeMap<String, u32>
}

impl SurfaceTexture {
    pub fn load(surface_yaml: &str) -> Result<SurfaceTexture, WorldError> {
        let content = read_file(surface_yaml)?;
        let file_info: FileInfo = serde_yaml::from_str(&content)?;
        let mut layer_map = BTreeMap::default();

        let mut builder = TextureBuilder::new_2d_array(file_info.get_path(), file_info.get_size_as_array());
        for (surface_name, coord) in file_info.get_coordinates().iter() {
            builder.add_array_element(*coord);
            if let Some(_existing) = layer_map.insert(surface_name.to_owned(), coord[2]) {
                warn!("Surface with name '{}' already existing!", surface_name);
            }
        }
        let texture_array = builder.finish()?;

        let surface_texture = SurfaceTexture {
            texture: texture_array,
            layer_map: layer_map
        };
        Ok(surface_texture)
    }

    pub fn get_layer(&self, surface_name: &str) -> u32 {
        match self.layer_map.get(surface_name) {
            Some(layer) => *layer,
            None => 0
        }
    }

    pub fn activate(&self) {
        self.texture.activate();
    }

    pub fn deactivate(&self) {
        self.texture.deactivate();
    }
}

#[derive(Deserialize)]
struct FileInfo {
    texture_size: u32,
    path: String,
    surface_coordinates: BTreeMap<String, [u32; 3]>
}

impl FileInfo {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_size_as_array(&self) -> [u32; 2] {
        [self.texture_size, self.texture_size]
    }

    pub fn get_coordinates(&self) -> &BTreeMap<String, [u32; 3]> {
        &self.surface_coordinates
    }
}

