use gl;
use gl::types::{ GLint, GLuint, GLsizei };
use image;
use image::GenericImageView;


use utility::Float;
use crate::{ GraphicsError, OpenglError, check_opengl_error };

pub struct Texture {
    texture_id: GLuint
}

impl Texture {
    pub fn new(img_path: &str) -> Result<Texture, GraphicsError> {

        debug!("Opening texture image '{}'", img_path);
        let texture_id = match image::open(img_path.clone())? {
            image::DynamicImage::ImageRgba8(img) => {
                let img_size = (img.width() as GLsizei, img.height() as GLsizei);
                let mipmaps = {
                    let dim = GLsizei::min(img_size.0, img_size.1) as Float;
                    dim.log(2.0) as GLsizei
                };
                let raw_data = img.into_raw();
                let id = create_texture(img_size, mipmaps, &raw_data)?;
                id
            },
            _ => { 
                return Err(GraphicsError::InvalidImageFormat(img_path.to_string()));
            } 
        };
        let texture = Self {
            texture_id: texture_id
        };
        Ok(texture)
    }

    pub fn activate(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }

    pub fn deactivate(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        debug!("Deleting texture id = {}", self.texture_id);
        delete_texture(self.texture_id);
        match check_opengl_error("gl::DeleteTextures") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}

fn create_texture(size: (GLsizei, GLsizei), mipmaps: GLsizei, img_data: &[u8]) -> Result<GLuint, OpenglError> {
    debug_assert!(size.0 > 0);
    debug_assert!(size.1 > 0);
    let mut id: GLuint = 0;
    unsafe { gl::GenTextures(1, &mut id); }
    check_opengl_error("gl::GenTextures")?;
    debug_assert!(id != 0);
    unsafe { gl::BindTexture(gl::TEXTURE_2D, id); }
    match check_opengl_error("gl::BindTexture") {
        Ok(_) => {},
        Err(e) => {
            delete_texture(id);
            return Err(e);
        }
    }
    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            mipmaps,
            gl::RGBA8 as GLsizei,
            size.0,
            size.1,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_data.as_ptr() as * const _
        );
    }
    Ok(id)
}

fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe { gl::DeleteTextures(1, &texture_id); }
}
