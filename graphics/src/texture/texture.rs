use std::convert::TryInto;
use gl;
use gl::types::{ GLint, GLuint, GLsizei };
use image;


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
                let img_size = [img.width() as GLsizei,
                                img.height() as GLsizei];
                let mipmaps = {
                    let dim = GLsizei::min(img_size[0], img_size[1]) as Float;
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
        texture.deactivate();
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

fn create_texture(size: [GLsizei; 2], mipmaps: GLsizei, img_data: &[u8]) -> Result<GLuint, OpenglError> {
    info!("Creating texture...");
    debug_assert!(size[0] >= 0 && size[0] <= gl::MAX_TEXTURE_SIZE.try_into().unwrap());
    debug_assert!(size[1] >= 0 && size[1] <= gl::MAX_TEXTURE_SIZE.try_into().unwrap());
    debug_assert!(mipmaps >= 0 && mipmaps <= (gl::MAX_TEXTURE_SIZE as f32).log(2.) as i32);
    debug_assert!(!img_data.as_ptr().is_null());

    info!("MAX = {}, mipmap_MAX = {}", gl::MAX_TEXTURE_SIZE, (gl::MAX_TEXTURE_SIZE as f32).log(2.).floor());

    let mut id: GLuint = 0;
    unsafe { gl::GenTextures(1, &mut id); }
    check_opengl_error("gl::GenTextures")?;
    debug!("Texture info: id = {}, size = {}x{}, mipmaps = {}", id, size[0], size[1], mipmaps);
    debug_assert!(id != 0);


    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, id);
    }
    if let Err(e) = check_opengl_error("gl::BindTexture") {
        delete_texture(id);
        return Err(e);
    }

    unsafe {
        gl::TexImage2D(
            gl::TEXTURE_2D,
            4,//mipmaps,    // TODO: check why mipmaps > 4 throw invalid value error
            gl::RGBA8.try_into().unwrap(),
            size[0].try_into().unwrap(),
            size[1].try_into().unwrap(),
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            img_data.as_ptr() as * const _
        );
    }
    if let Err(e) = check_opengl_error("gl::TexImage2D") {
        delete_texture(id);
        return Err(e);
    }

    unsafe {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        if let Err(e) = check_opengl_error("gl::TexParameteri") {
            delete_texture(id);
            return Err(e);
        }
        gl::GenerateMipmap(gl::TEXTURE_2D);
        if let Err(e) = check_opengl_error("gl::GenerateMipmap") {
            delete_texture(id);
            return Err(e);
        }
    }

    Ok(id)
}

fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe { gl::DeleteTextures(1, &texture_id); }
}
