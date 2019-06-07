use std::collections::BTreeMap;

use gl;

use crate::{ Texture, GraphicsError };
use super::{ Orientation, TextureType, utility::* };

pub struct TextureBuilder {
    img_path: String,
    texture_type: TextureType,
}


impl TextureBuilder {
    pub fn new_2d(img_path: &str) -> TextureBuilder {
        TextureBuilder::new(
            img_path,
            TextureType::Single2D
        )
    }
    pub fn new_2d_array(img_path: &str, sub_size: [u32; 2]) -> TextureBuilder {
        TextureBuilder::new(
            img_path,
            TextureType::Array2D { index_list: Vec::new(), size: sub_size }
        )
    }
    pub fn new_cube_map(img_path: &str, cube_size: u32) -> TextureBuilder {
        TextureBuilder::new(
            img_path,
            TextureType::CubeMap { origin_map: BTreeMap::new(), size: cube_size }
        )
    }

    fn new(img_path: &str, texture_type: TextureType) -> TextureBuilder {
        TextureBuilder {
            img_path: img_path.into(),
            texture_type: texture_type
        }
    }

    pub fn add_array_element(&mut self, index: [u32; 3]) {
        if let TextureType::Array2D { index_list, .. } = &mut self.texture_type {
            index_list.push(index);
        } else {
            warn!("Wanted to add array element to non 2d array texture");
        }
    }

    pub fn add_cube_element(&mut self, origin: [u32; 2], orientation: Orientation) {
        if let TextureType::CubeMap { origin_map, .. } = &mut self.texture_type {
            match orientation {
                Orientation::Right => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_POSITIVE_X, origin);
                },
                Orientation::Left => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_NEGATIVE_X, origin);
                },
                Orientation::Top => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_POSITIVE_Y, origin);
                },
                Orientation::Bottom => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_NEGATIVE_Y, origin);
                },
                Orientation::Back => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_POSITIVE_Z, origin);
                },
                Orientation::Front => {
                    origin_map.insert(gl::TEXTURE_CUBE_MAP_NEGATIVE_Z, origin);
                }
            }
        } else {
            warn!("Wanted to add array element to non cubemap texture");
        }
    }

    pub fn finish(&self) -> Result<Texture, GraphicsError> {
        let img = load_image(&self.img_path)?;
        let texture_size = get_texture_size(&self.texture_type, &img);

        let id = initialize_texture(&self.texture_type)?;

        if let Err(e) = create_texture_storage(&self.texture_type, texture_size) {
            delete_texture(id);
            return Err(e.into());
        }

        if let Err(e) = fill_texture(&self.texture_type, img) {
            delete_texture(id);
            return Err(e.into());
        }

        if let Err(e) = unbind_texture(&self.texture_type) {
            delete_texture(id);
            return Err(e.into());
        }

        let texture = Texture::new(id, get_opengl_texture_type(&self.texture_type));
        Ok(texture)
    }
}
