use gl;
use gl::types::{ GLint, GLuint };
use glm::Matrix4;

use crate::utility::Float;
use crate::graphics::{ check_opengl_error };
use super::ShaderProgramError;

pub struct ShaderProgram {
    id: GLuint, 
    mvp_handle: GLint,
    texture_array_handle: GLint
}

impl ShaderProgram {
    
    pub fn new(program_id: GLuint) -> Result<ShaderProgram, ShaderProgramError> {
        debug_assert!(program_id != 0);
        let program = Self {
            id: program_id,
            mvp_handle: get_resource_handle(program_id, "MVP")?,
            texture_array_handle: get_resource_handle(program_id, "textureArray")?
        };
        program.use_program();
        unsafe { gl::Uniform1i(program.texture_array_handle, 0) }
        check_opengl_error("gl::Uniform1i")?;
        Ok(program)
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mvp_matrix(&self, mvp_matrix: &Matrix4<Float>) -> Result<(), ShaderProgramError> {
        unsafe {
            gl::UniformMatrix4fv(self.mvp_handle, 1, gl::FALSE, mvp_matrix.as_array().as_ptr() as * const Float);
        }
        check_opengl_error("gl::UniformMatrix4fv")?;
        Ok(())
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        debug!("Deleting shader program");
        unsafe { gl::DeleteProgram(self.id); }
        match check_opengl_error("gl::DeleteProgram") {
            Ok(_) => {},
            Err(e) => error!("{}", e)
        }
    }
}

fn get_resource_handle(program_id: GLuint, resource_name: &str) -> Result<GLint, ShaderProgramError> {
    let res_name_zero_term = resource_name.to_string() + "\0";
    let handle: GLint = unsafe {
        gl::GetUniformLocation(program_id, res_name_zero_term.as_ptr() as *const _)
    };
    if handle == -1 {
        check_opengl_error("gl::GetUniformLocation")?;
        return Err(ShaderProgramError::FunctionFailure("gl::GetUniformLocation".to_string()));
    }
    Ok(handle)
}
