use std::collections::BTreeMap;

use serde::Deserialize;

use graphics::{ Texture, TextureBuilder };
use utility::read_file;
use crate::WorldError;
use super::{ TerrainSet, Terrain, TerrainType };

pub struct SurfaceTexture {
    texture: Texture,
    terrain_set: TerrainSet
}

impl SurfaceTexture {
    pub fn load(surface_yaml: &str) -> Result<SurfaceTexture, WorldError> {
        let content = read_file(surface_yaml)?;
        let file_info: FileInfo = serde_yaml::from_str(&content)?;
        let mut terrain_set = TerrainSet::default();

        let mut builder = TextureBuilder::new_2d_array(file_info.get_path(), file_info.get_size_as_array());
        for (terrain_type, coord) in file_info.get_coordinates().iter() {
            builder.add_array_element(*coord);
            if let Some(_existing) = terrain_set.insert(*terrain_type, Terrain::new(*terrain_type, coord[2])) {
                warn!("Terrain of type '{}' already existing!", terrain_type);
            }
        }
        let texture_array = builder.finish()?;

        let surface_texture = SurfaceTexture {
            texture: texture_array,
            terrain_set: terrain_set
        };
        Ok(surface_texture)
    }

    pub fn get_terrain_set(&self) -> &TerrainSet {
        &self.terrain_set
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
    surface_coordinates: BTreeMap<TerrainType, [u32; 3]>
}

impl FileInfo {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_size_as_array(&self) -> [u32; 2] {
        [self.texture_size, self.texture_size]
    }

    pub fn get_coordinates(&self) -> &BTreeMap<TerrainType, [u32; 3]> {
        &self.surface_coordinates
    }
}

