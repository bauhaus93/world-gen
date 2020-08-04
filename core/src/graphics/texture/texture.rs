use gl;
use gl::types::{GLenum, GLfloat, GLint, GLsizei, GLuint};
use glm::Vector4;
use std::collections::VecDeque;

use image::{GenericImageView, RgbaImage};

use crate::file::read_image_rgb;
use crate::graphics::{check_opengl_error, GraphicsError, OpenglError};
use crate::{Point2f, Point2i, Point3i};

pub struct Texture {
    id: GLuint,
    size: Point3i,
    tex_type: GLenum,
    format: GLenum,
    channels: GLenum,
    channel_type: GLenum,
    mipmaps: bool,
}

impl Texture {
    pub fn new(
        id: GLuint,
        size: Point3i,
        tex_type: GLenum,
        format: GLenum,
        mipmaps: bool,
    ) -> Texture {
        let (channels, channel_type) = match format {
            gl::RGBA8 => (gl::RGBA, gl::UNSIGNED_BYTE),
            gl::RGBA32F => (gl::RGBA, gl::FLOAT),
            gl::R32F => (gl::RED, gl::FLOAT),
            gl::RG32F => (gl::RG, gl::FLOAT),
            gl::RG32I => (gl::RG, gl::INT),
            _ => {
                error!("Unhandled texture format");
                unreachable!();
            }
        };
        debug_assert!(size[0] > 0 && size[1] > 0 && size[2] >= 0);
        Texture {
            id: id,
            size: size,
            tex_type: tex_type,
            format: format,
            channels: channels,
            channel_type: channel_type,
            mipmaps: mipmaps,
        }
    }

    pub fn activate(&self, slot: u8) {
        let texture_slot = match slot {
            0 => gl::TEXTURE0,
            1 => gl::TEXTURE1,
            2 => gl::TEXTURE2,
            _ => unreachable!("Texture slot too high"),
        };
        unsafe {
            gl::ActiveTexture(texture_slot);
            gl::BindTexture(self.tex_type, self.id);
        }
        // TODO: maybe add check_opengl_error
    }

    pub fn deactivate(&self) {
        unsafe { gl::BindTexture(self.tex_type, 0) }
        // TODO: maybe add check_opengl_error
    }

    pub fn read_data_rgba32f(&self) -> Result<Vec<Vector4<GLfloat>>, OpenglError> {
        let size = 4 * self.size[0] as usize * self.size[1] as usize * self.size[2] as usize;
        let mut data: Vec<GLfloat> = Vec::with_capacity(size);
        data.resize(size, 0.);

        self.activate(0);
        unsafe {
            gl::GetTexImage(
                self.tex_type,
                0,
                gl::RGBA,
                gl::FLOAT,
                data.as_mut_ptr() as *mut _,
            );
            check_opengl_error("glGetTexImage")?;
        }
        self.deactivate();
        let mut data_vec = Vec::new();
        for i in 0..data.len() / 4 {
            data_vec.push(Vector4::new(
                data[4 * i],
                data[1 + 4 * i],
                data[2 + 4 * i],
                data[3 + 4 * i],
            ));
        }
        data_vec.reverse();
        Ok(data_vec)
    }

    pub fn fill_with_image(&self, img_path: &str) -> Result<(), GraphicsError> {
        let img = read_image_rgb(img_path)?;
        let (width, height) = img.dimensions();
        let data: Vec<u8> = img.into_raw();
        unsafe {
            self.activate(0);
            match self.tex_type {
                t if t == gl::TEXTURE_2D => {
                    gl::TexSubImage2D(
                        t,
                        0,
                        0,
                        0,
                        width as GLsizei,
                        height as GLsizei,
                        gl::RGB,
                        gl::UNSIGNED_BYTE,
                        data.as_ptr() as *const _,
                    );
                    check_opengl_error("TexSubImage2D")?;
                    handle_new_2d_image(self.mipmaps)?;    
                }
                _ => {
                    unimplemented!();
                }
            }
            self.deactivate();
        }
        Ok(())
    }

    pub fn write_data_rgba32f(&self, data: &[Vector4<GLfloat>]) -> Result<(), OpenglError> {
        /*self.activate(0);
        let data_floats: Vec<GLfloat> = data.iter().flat_map(|v| vec!(v[0], v[1], v[2], v[3])).collect();
        unsafe {
            match self.tex_type {
                t if t == gl::TEXTURE_1D => {
                    gl::TexImage1D(
                        t,
                        0,
                        self.format as GLint,
                        data_floats.len() as GLsizei,
                        0,
                        gl::RGBA,
                        gl::FLOAT,
                        data.as_ptr() as *const _,
                    );
                    check_opengl_error("glTexImage1D")?;
                }
                _ => unreachable!("Not yet handeled"),
            }
        }
        self.deactivate();*/
        Ok(())
    }

    pub fn load_cube_image(
        &mut self,
        orientation: Point3i,
        img_origin: Point2i,
        img: &RgbaImage,
    ) -> Result<(), GraphicsError> {
        if self.tex_type == gl::TEXTURE_CUBE_MAP && self.format == gl::RGBA8 {
            let gl_orientation = match orientation {
                o if o[0] > 0 && o[1] == 0 && o[2] == 0 => gl::TEXTURE_CUBE_MAP_POSITIVE_X,
                o if o[0] < 0 && o[1] == 0 && o[2] == 0 => gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
                o if o[0] == 0 && o[1] > 0 && o[2] == 0 => gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
                o if o[0] == 0 && o[1] < 0 && o[2] == 0 => gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
                o if o[0] == 0 && o[1] == 0 && o[2] > 0 => gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
                o if o[0] == 0 && o[1] == 0 && o[2] < 0 => gl::TEXTURE_CUBE_MAP_NEGATIVE_Z,
                _ => {
                    warn!("Invalid orientation given for cube image: {}", orientation);
                    unreachable!();
                }
            };
            self.activate(0);
            let sub_img = img
                .view(
                    img_origin[0] as u32,
                    img_origin[1] as u32,
                    self.size[0] as u32,
                    self.size[1] as u32,
                )
                .to_image();
            let pixels: Vec<u8> = sub_img.into_raw();
            unsafe {
                gl::TexSubImage2D(
                    gl_orientation,
                    0,
                    0,
                    0,
                    self.size[0],
                    self.size[1],
                    self.channels,
                    self.channel_type,
                    pixels.as_ptr() as *const _,
                );
            }
            check_opengl_error("gl::TexSubImage2D")?;
            handle_new_cubemap_image(self.mipmaps)?;
            self.deactivate();
        } else {
            warn!("Wanted to load cube image for non-cubemap texture");
        }
        Ok(())
    }
}

fn handle_new_cubemap_image(mipmaps: bool) -> Result<(), OpenglError> {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_EDGE as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_EDGE as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_WRAP_R,
            gl::CLAMP_TO_EDGE as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_NEAREST as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_CUBE_MAP,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as GLint,
        );
        check_opengl_error("gl::TexParameteri")?;
        if mipmaps {
            gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
            check_opengl_error("gl::GenerateMipmap")?;
        }
    }
    Ok(())
}

fn handle_new_2d_image(mipmaps: bool) -> Result<(), OpenglError> {
    unsafe {
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::REPEAT as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::REPEAT as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_R,
            gl::REPEAT as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::NEAREST_MIPMAP_NEAREST as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            gl::NEAREST as GLint,
        );
        check_opengl_error("gl::TexParameteri")?;
        if mipmaps {
            gl::GenerateMipmap(gl::TEXTURE_2D);
            check_opengl_error("gl::GenerateMipmap")?;
        }
    }
    Ok(())
}

impl Drop for Texture {
    fn drop(&mut self) {
        debug!(
            "Deleting texture: id = {}, type = {}",
            self.id, self.tex_type
        );
        if let Err(e) = delete_texture(self.id) {
            error!("{}", e);
        }
    }
}

fn delete_texture(texture_id: GLuint) -> Result<(), OpenglError> {
    debug_assert!(texture_id != 0);
    unsafe {
        gl::DeleteTextures(1, &texture_id);
    }
    check_opengl_error("gl::DeleteTextures")?;
    Ok(())
}
