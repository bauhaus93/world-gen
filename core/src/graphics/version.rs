use std::string::FromUtf8Error;
use gl;

use crate::graphics::opengl_string::get_string;

pub fn get_opengl_version() -> Result<String, FromUtf8Error> {
    get_string(gl::VERSION)
}
