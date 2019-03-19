#![feature(try_from)]

#[macro_use]
extern crate log;
extern crate gl;
extern crate glutin;
extern crate glm;
extern crate image;
extern crate num_traits;

extern crate utility;

pub mod shader;
pub mod texture;
pub mod mesh;
pub mod projection;
pub mod transformation;
pub mod version;
pub mod graphics_error;
mod opengl_error;
mod opengl_string;

pub use self::shader::ShaderProgram;
pub use self::shader::ShaderProgramBuilder;
pub use self::texture::TextureArray;
pub use self::texture::TextureArrayBuilder;
pub use self::mesh::Mesh;
pub use self::mesh::triangle::Triangle;
pub use self::projection::Projection;
pub use self::transformation::{ create_transformation_matrix, create_translation_matrix, create_rotation_matrix, create_scale_matrix, create_direction };
pub use self::graphics_error::GraphicsError;
pub use self::opengl_error::{ OpenglError, check_opengl_error };
pub use self::version::get_opengl_version;
