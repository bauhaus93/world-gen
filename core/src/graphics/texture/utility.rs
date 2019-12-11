use std::collections::BTreeMap;

use gl;
use gl::types::{ GLenum, GLint, GLuint, GLsizei };
use image;
use image::{ GenericImageView, RgbaImage };

use crate::{ Float, graphics::{GraphicsError, OpenglError, check_opengl_error }};
use super::TextureType;

pub fn load_image(path: &str) -> Result<RgbaImage, GraphicsError> {
    debug!("Opening image '{}'", path);
    let img = match image::open(path.clone())? {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => { 
            return Err(GraphicsError::InvalidImageFormat(path.into()));
        } 
    };
    Ok(img)
}

pub fn initialize_texture(texture_type: &TextureType) -> Result<GLuint, OpenglError> {
    trace!("Creating new texture");
    let mut id: GLuint = 0;
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::GenTextures(1, &mut id);
    }
    check_opengl_error("gl::GenTextures")?;
    debug_assert!(id != 0);
    unsafe {
        gl::BindTexture(get_opengl_texture_type(texture_type), id);
    }
    if let Err(e) = check_opengl_error("gl::BindTexture") {
        delete_texture(id);
        return Err(e);
    }
    Ok(id)
}

// texture must already be bound
pub fn create_texture_storage(texture_type: &TextureType, size: [GLsizei; 2]) -> Result<(), GraphicsError> {
    let mipmaps = calculate_mipmaps(size);
    match texture_type {
        TextureType::Single2D => {
            trace!("Creating texture storage for gl::TEXTURE_2D: size = ({}x{}), mipmaps = {}", size[0], size[1], mipmaps);
            unsafe {
                gl::TexStorage2D(
                    gl::TEXTURE_2D,
                    mipmaps,
                    gl::RGBA8,
                    size[0],
                    size[1],
                );
            }
            check_opengl_error("gl::TexStorage2D(gl::TEXTURE_2D)")?;
        },
        TextureType::Array2D { index_list, .. } => {
            unsafe {
                trace!("Creating texture storage for gl::TEXTURE_2D_ARRAY: size = ({}x{}x{}), mipmaps = {}", size[0], size[1], index_list.len(), mipmaps);
                gl::TexStorage3D(
                    gl::TEXTURE_2D_ARRAY,
                    mipmaps,
                    gl::RGBA8,
                    size[0],
                    size[1],
                    index_list.len() as i32
                );
            }
            check_opengl_error("gl::TexStorage3D(gl::TEXTURE_2D_ARRAY)")?;
        },
        TextureType::CubeMap { .. } => {
            trace!("Creating texture storage for gl::TEXTURE_CUBE_MAP: size = ({}x{}), mipmaps = {}", size[0], size[1], mipmaps);
            unsafe {
                gl::TexStorage2D(
                    gl::TEXTURE_CUBE_MAP,
                    mipmaps,
                    gl::RGBA8,
                    size[0],
                    size[1],
                );
            }
            check_opengl_error("gl::TexStorage2D(gl::TEXTURE_CUBE_MAP")?;
        }
    }
    trace!("Sucessfully created texture storage");

    Ok(())
}

pub fn fill_texture(texture_type: &TextureType, img: RgbaImage) -> Result<(), GraphicsError> {
    match texture_type {
        TextureType::Single2D => {
            trace!("Filling gl::TEXTURE_2D");
            unsafe {
                gl::TexSubImage2D(
                    gl::TEXTURE_2D,
                    0,
                    0, 0,
                    img.width() as GLsizei, img.height() as GLsizei,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    img.into_raw().as_ptr() as * const _
                );
            }
            check_opengl_error("gl::TexSubImage2D(gl::TEXTURE_2D)")?;
        },
        TextureType::Array2D { index_list, size } => {
            trace!("Filling gl::TEXTURE_2D_ARRAY");
            add_subimages(&img, *size, index_list)?;
        },
        TextureType::CubeMap { index_map, size } => {
            trace!("Filling gl::TEXTURE_CUBE_MAP");
            add_cube_images(&img, *size, index_map)?;
        }
    }
    trace!("Successfully filled texture");
    Ok(())
}

pub fn add_subimages(img: &RgbaImage, sub_size: [u32; 2], index_list: &[[u32; 3]])  -> Result<(), GraphicsError>{ 
    for index in index_list.iter() {
        let origin = [index[0] * sub_size[0],
                      index[1] * sub_size[1]];
        trace!("Adding subimage, origin = {}/{}, layer = {}", origin[0], origin[1], index[2]);
        let sub_img = img.view(origin[0], origin[1], sub_size[0], sub_size[1]).to_image();
        let pixels: Vec<u8> = sub_img.into_raw();
        add_subimage(
            [sub_size[0] as GLsizei, sub_size[1] as GLsizei],
            index[2] as GLsizei,
            &pixels
        )?;
    }
    Ok(())
}

pub fn add_subimage(size: [GLsizei; 2], layer: GLsizei, sub_image: &[u8]) -> Result<(), OpenglError> {
    unsafe {
        gl::TexSubImage3D(
            gl::TEXTURE_2D_ARRAY,
            0,
            0, 0, layer,
            size[0], size[1], 1,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            sub_image.as_ptr() as * const _
        );
        check_opengl_error("gl::TexSubImage3D")?;
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        check_opengl_error("gl::TexParameteri")?;
        gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        check_opengl_error("gl::GenerateMipmap")?;
    }
    Ok(())
}

pub fn add_cube_images(img: &RgbaImage, cube_size: u32, index_map: &BTreeMap<GLenum, [u32; 2]>) -> Result<(), GraphicsError> {
    for (orientation, index) in index_map {
        let origin = [index[0] * cube_size,
                      index[1] * cube_size];
        trace!("Adding cube image, origin = {}/{}, size = {}/{}", origin[0], origin[1], cube_size, cube_size);
        let sub_img = img.view(origin[0], origin[1], cube_size, cube_size).to_image();
        let pixels: Vec<u8> = sub_img.into_raw();
        add_cube_image(
            cube_size as GLsizei,
            *orientation,
            &pixels)
        ?; 
    }
    Ok(())
}

pub fn add_cube_image(cube_size: GLsizei, orientation: GLenum, cube_image: &[u8]) -> Result<(), GraphicsError> {
    unsafe {
        gl::TexSubImage2D(
            orientation,
            0,
            0, 0,
            cube_size, cube_size,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            cube_image.as_ptr() as * const _
        );
        check_opengl_error("gl::TexSubImage2D")?;
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        check_opengl_error("gl::TexParameteri")?;
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
        check_opengl_error("gl::GenerateMipmap")?;
    }
    Ok(())
}

pub fn get_opengl_texture_type(texture_type: &TextureType) -> GLenum {
    match texture_type {
        TextureType::Single2D => gl::TEXTURE_2D,
        TextureType::Array2D { .. } => gl::TEXTURE_2D_ARRAY,
        TextureType::CubeMap { .. } => gl::TEXTURE_CUBE_MAP
    }
}

pub fn get_texture_size(texture_type: &TextureType, texture_img: &RgbaImage) -> [GLsizei; 2] {
    match texture_type {
        TextureType::Single2D => [texture_img.width() as GLsizei,
                                  texture_img.height() as GLsizei],
        TextureType::Array2D { size, .. } => [size[0] as GLsizei,
                                              size[1] as GLsizei],
        TextureType::CubeMap { size, ..} => [*size as GLsizei,
                                             *size as GLsizei],
    }
}

pub fn calculate_mipmaps(texture_size: [GLsizei; 2]) -> GLsizei {
    (GLsizei::min(texture_size[0], texture_size[1]) as Float)
    .log(2.) as GLsizei
}

pub fn unbind_texture(texture_type: &TextureType) -> Result<(), OpenglError> {
    unsafe {
        gl::BindTexture(get_opengl_texture_type(texture_type), 0);
    }
    check_opengl_error("gl::BindTexture")
}


pub fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe { gl::DeleteTextures(1, &texture_id); }
}
