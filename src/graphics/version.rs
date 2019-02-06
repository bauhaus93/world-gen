use std::string::FromUtf8Error;
use gl;

use super::utility::opengl_get_string;

pub fn get_opengl_version() -> Result<String, FromUtf8Error> {
    opengl_get_string(gl::VERSION)
}
