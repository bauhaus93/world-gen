use std::collections::BTreeMap;
use std::convert::TryInto;

use gl;
use gl::types::{ GLenum, GLint, GLuint, GLsizei };
use image;
use image::{ GenericImageView, RgbaImage };

use utility::Float;
use crate::{ Texture, GraphicsError, OpenglError, check_opengl_error };

pub struct TextureBuilder {
    img_path: String,
    texture_type: TextureType,
}

enum TextureType {
    Single2D,
    Array2D { index_list: Vec<[u32; 3]>, size: [u32; 2] },
    CubeMap { origin_map: BTreeMap<GLenum, [u32; 2]>, size: [u32; 2] }
}

pub enum Orientation {
    Right,
    Left,
    Top,
    Bottom,
    Back,
    Front
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
    pub fn new_cube_map(img_path: &str, sub_size: [u32; 2]) -> TextureBuilder {
        TextureBuilder::new(
            img_path,
            TextureType::CubeMap { origin_map: BTreeMap::new(), size: sub_size }
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

    fn finish(&mut self) -> Result<Texture, GraphicsError> {
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


fn load_image(path: &str) -> Result<RgbaImage, GraphicsError> {
    debug!("Opening image '{}'", path);
    let img = match image::open(path.clone())? {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => { 
            return Err(GraphicsError::InvalidImageFormat(path.into()));
        } 
    };
    Ok(img)
}


fn initialize_texture(texture_type: &TextureType) -> Result<GLuint, OpenglError> {
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
fn create_texture_storage(texture_type: &TextureType, size: [GLsizei; 2]) -> Result<(), GraphicsError> {
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

fn fill_texture(texture_type: &TextureType, img: RgbaImage) -> Result<(), GraphicsError> {
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
        TextureType::CubeMap { origin_map, size } => {
            trace!("Filling gl::TEXTURE_CUBE_MAP");
            add_cube_images(&img, *size, origin_map)?;
        }
    }
    trace!("Successfully filled texture");
    Ok(())
}

fn add_subimages(img: &RgbaImage, sub_size: [u32; 2], index_list: &[[u32; 3]])  -> Result<(), GraphicsError>{ 
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

fn add_subimage(size: [GLsizei; 2], layer: GLsizei, sub_image: &[u8]) -> Result<(), OpenglError> {
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

fn add_cube_images(img: &RgbaImage, sub_size: [u32; 2], origin_map: &BTreeMap<GLenum, [u32; 2]>) -> Result<(), GraphicsError> {
    for (orientation, origin) in origin_map {
        trace!("Adding cube image, origin = {}/{}, size = {}/{}", origin[0], origin[1], sub_size[0], sub_size[1]);
        let sub_img = img.view(origin[0], origin[1], sub_size[0], sub_size[1]).to_image();
        let pixels: Vec<u8> = sub_img.into_raw();
        add_cube_image(
            [sub_size[0] as GLsizei,
             sub_size[1] as GLsizei],
            *orientation,
            &pixels)
        ?; 
    }
    Ok(())
}

fn add_cube_image(size: [GLsizei; 2], orientation: GLenum, cube_image: &[u8]) -> Result<(), GraphicsError> {
    unsafe {
        gl::TexSubImage2D(
            orientation,
            0,
            0, 0,
            size[0], size[1],
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            cube_image.as_ptr() as * const _
        );
        check_opengl_error("gl::TexSubImage2D")?;
        gl::TexParameteri(orientation, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(orientation, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(orientation, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as GLint);
        gl::TexParameteri(orientation, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);
        check_opengl_error("gl::TexParameteri")?;
        gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
        check_opengl_error("gl::GenerateMipmap")?;
    }
    Ok(())
}

fn get_opengl_texture_type(texture_type: &TextureType) -> GLenum {
    match texture_type {
        TextureType::Single2D => gl::TEXTURE_2D,
        TextureType::Array2D { .. } => gl::TEXTURE_2D_ARRAY,
        TextureType::CubeMap { .. } => gl::TEXTURE_CUBE_MAP
    }
}

fn get_texture_size(texture_type: &TextureType, texture_img: &RgbaImage) -> [GLsizei; 2] {
    match texture_type {
        TextureType::Single2D => [texture_img.width().try_into().unwrap(),
                                  texture_img.height().try_into().unwrap()],
        TextureType::Array2D { size, .. } => [size[0].try_into().unwrap(),
                                              size[1].try_into().unwrap()],
        TextureType::CubeMap { size, ..} => [size[0].try_into().unwrap(),
                                             size[1].try_into().unwrap()],
    }
}

fn calculate_mipmaps(texture_size: [GLsizei; 2]) -> GLsizei {
    (GLsizei::min(texture_size[0], texture_size[1]) as Float)
    .log(2.) as GLsizei
}

fn unbind_texture(texture_type: &TextureType) -> Result<(), OpenglError> {
    unsafe {
        gl::BindTexture(get_opengl_texture_type(texture_type), 0);
    }
    check_opengl_error("gl::BindTexture")
}


fn delete_texture(texture_id: GLuint) {
    debug_assert!(texture_id != 0);
    unsafe { gl::DeleteTextures(1, &texture_id); }
}