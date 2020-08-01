use std::collections::BTreeMap;

use gl;
use gl::types::{GLenum, GLint, GLsizei, GLuint};

use crate::{
    graphics::{check_opengl_error, GraphicsError, OpenglError},
    Float, Point3i,
};

pub fn initialize_texture(texture_type: GLenum) -> Result<GLuint, OpenglError> {
    trace!("Creating new texture");
    let mut id: GLuint = 0;
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::GenTextures(1, &mut id);
    }
    check_opengl_error("gl::GenTextures")?;
    debug_assert!(id != 0);
    unsafe {
        gl::BindTexture(texture_type, id);
    }
    if let Err(e) = check_opengl_error("gl::BindTexture") {
        delete_texture(id);
        return Err(e);
    }
    Ok(id)
}

// texture must already be bound
pub fn create_texture_storage(
    texture_type: GLenum,
    size: Point3i,
    texture_format: GLenum,
    use_mipmaps: bool,
) -> Result<(), GraphicsError> {
    let mipmaps = match use_mipmaps {
        true => calculate_mipmaps(size),
        false => 1,
    };
    match texture_type {
        t if t == gl::TEXTURE_1D => {
            unsafe {
                gl::TexStorage1D(t, mipmaps, texture_format, size[0] as GLsizei);
            }
            check_opengl_error("gl::TexStorage1D(gl::TEXTURE_1D)")?;
        }
        t if t == gl::TEXTURE_2D => {
            unsafe {
                gl::TexStorage2D(
                    t,
                    mipmaps,
                    texture_format,
                    size[0] as GLsizei,
                    size[1] as GLsizei,
                );
            }
            check_opengl_error("gl::TexStorage2D(gl::TEXTURE_2D)")?;
        }
        t if t == gl::TEXTURE_2D_ARRAY => {
            unsafe {
                gl::TexStorage3D(
                    t,
                    mipmaps,
                    texture_format,
                    size[0] as GLsizei,
                    size[1] as GLsizei,
                    size[2] as i32,
                );
            }
            check_opengl_error("gl::TexStorage3D(gl::TEXTURE_2D_ARRAY)")?;
        }
        t if t == gl::TEXTURE_CUBE_MAP => {
            unsafe {
                gl::TexStorage2D(
                    t,
                    mipmaps,
                    texture_format,
                    size[0] as GLsizei,
                    size[1] as GLsizei,
                );
            }
            check_opengl_error("gl::TexStorage2D(gl::TEXTURE_CUBE_MAP")?;
        }
        t => {
            error!("Unknown texture type");
        }
    }
    trace!("Sucessfully created texture storage");

    Ok(())
}

pub fn add_subimage(
    size: [GLsizei; 2],
    layer: GLsizei,
    sub_image: &[u8],
) -> Result<(), OpenglError> {
    unsafe {
        gl::TexSubImage3D(
            gl::TEXTURE_2D_ARRAY,
            0,
            0,
            0,
            layer,
            size[0],
            size[1],
            1,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            sub_image.as_ptr() as *const _,
        );
        check_opengl_error("gl::TexSubImage3D")?;
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_NEAREST as GLint,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        check_opengl_error("gl::TexParameteri")?;
        gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        check_opengl_error("gl::GenerateMipmap")?;
    }
    Ok(())
}

pub fn calculate_mipmaps(texture_size: Point3i) -> GLsizei {
    match texture_size {
        t if t[0] > 0 && t[1] > 0 && t[2] > 0 => {
            (i32::min(i32::min(texture_size[0], texture_size[1]), texture_size[2]) as Float).log(2.)
                as GLsizei
        }
        t if t[0] > 0 && t[1] > 0 => {
            (i32::min(texture_size[0], texture_size[1]) as Float).log(2.) as GLsizei
        }
        t if t[0] > 0 => (texture_size[0] as Float).log(2.) as GLsizei,
        _ => 0,
    }
}

pub fn unbind_texture(texture_type: GLenum) -> Result<(), OpenglError> {
    unsafe {
        gl::BindTexture(texture_type, 0);
    }
    check_opengl_error("gl::BindTexture")
}

pub fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe {
        gl::DeleteTextures(1, &texture_id);
    }
}
