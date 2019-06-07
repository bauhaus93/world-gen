use gl;
use gl::types::{ GLuint, GLenum };

use crate::check_opengl_error;

pub struct Texture {
    id: GLuint,
    tex_type: GLenum
}

impl Texture {
    pub fn new(id: GLuint, tex_type: GLenum) -> Texture {
        Texture {
            id: id,
            tex_type: tex_type
        }
    }

    pub fn activate(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(self.tex_type, self.id);
        }
        // TODO: maybe add check_opengl_error
    }

    pub fn deactivate(&self) {
        unsafe { gl::BindTexture(self.tex_type, 0) }
        // TODO: maybe add check_opengl_error
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        debug!("Deleting texture: id = {}, type = {}", self.id, self.tex_type);
        delete_texture(self.id);
        if let Err(e) = check_opengl_error("gl::DeleteTextures") {
            error!("{}", e);
        }
    }
}

fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe { gl::DeleteTextures(1, &texture_id); }
}
