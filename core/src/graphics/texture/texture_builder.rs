use gl;
use gl::types::GLenum;

use super::utility::*;
use crate::graphics::{GraphicsError, Texture};
use crate::{Point2i, Point3i};

pub struct TextureBuilder {
    size: Point3i,
    texture_type: GLenum,
    format: Option<GLenum>,
    use_mipmaps: Option<bool>,
}

impl TextureBuilder {
    pub fn new_1d(size: i32) -> TextureBuilder {
        TextureBuilder::new(gl::TEXTURE_1D, Point3i::new(size, 0, 0))
    }
    pub fn new_2d(size: Point2i) -> TextureBuilder {
        TextureBuilder::new(gl::TEXTURE_2D, size.extend(0))
    }
    pub fn new_2d_array(size: Point3i) -> TextureBuilder {
        TextureBuilder::new(gl::TEXTURE_2D_ARRAY, size)
    }
    pub fn new_cube_map(size: i32) -> TextureBuilder {
        TextureBuilder::new(gl::TEXTURE_CUBE_MAP, Point2i::from_scalar(size).extend(0))
    }

    fn new(texture_type: GLenum, size: Point3i) -> TextureBuilder {
        TextureBuilder {
            size: size,
            texture_type: texture_type,
            format: None,
            use_mipmaps: None,
        }
    }

    pub fn use_mipmaps(mut self) -> Self {
        self.use_mipmaps = Some(true);
        self
    }

    pub fn format_rgba8(mut self) -> Self {
        self.format = Some(gl::RGBA8);
        self
    }
    pub fn format_rgba32f(mut self) -> Self {
        self.format = Some(gl::RGBA32F);
        self
    }
    pub fn format_r32f(mut self) -> Self {
        self.format = Some(gl::R32F);
        self
    }

    pub fn format_rg32f(mut self) -> Self {
        self.format = Some(gl::RG32F);
        self
    }

    pub fn format_rg32i(mut self) -> Self {
        self.format = Some(gl::RG32I);
        self
    }

    pub fn finish(self) -> Result<Texture, GraphicsError> {
        let id = initialize_texture(self.texture_type)?;

        let use_mipmaps = self.use_mipmaps.unwrap_or(false);
        let format = self.format.unwrap_or(gl::RGBA8);

        if let Err(e) = create_texture_storage(self.texture_type, self.size, format, use_mipmaps) {
            delete_texture(id);
            return Err(e.into());
        }

        if let Err(e) = unbind_texture(self.texture_type) {
            delete_texture(id);
            return Err(e.into());
        }

        let texture = Texture::new(id, self.size, self.texture_type, format, use_mipmaps);
        Ok(texture)
    }
}
