use std::ffi::CStr;
use std::string::FromUtf8Error;

use gl;
use gl::types::GLenum;

pub fn opengl_get_string(name: GLenum) -> Result<String, FromUtf8Error> {
    let data_vec = unsafe {
        CStr::from_ptr(gl::GetString(name) as *const _).to_bytes().to_vec()
    };
    Ok(String::from_utf8(data_vec)?)
}


