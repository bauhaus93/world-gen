use gl;
use gl::types::GLuint;

use crate::check_opengl_error;

pub struct TextureArray {
    texture_id: GLuint
}

impl TextureArray {
    pub fn new(texture_id: GLuint) -> TextureArray {
        debug_assert!(texture_id != 0);
        TextureArray {
            texture_id: texture_id,
        }
    }
    pub fn activate(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_id);
        }
    }

    pub fn deactivate(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D_ARRAY, 0) }
    }
}

impl Drop for TextureArray {
    fn drop(&mut self) {
        debug!("Deleting texture id = {}", self.texture_id);
        unsafe { gl::DeleteTextures(1, &self.texture_id); }
        match check_opengl_error("gl::DeleteTextures") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}
