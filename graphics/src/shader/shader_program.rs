use std::collections::BTreeMap;
use gl;
use gl::types::{ GLint, GLuint };
use glm::{ Matrix4, Vector3 };

use utility::Float;
use crate::check_opengl_error;
use crate::shader::ShaderProgramError;

pub struct ShaderProgram {
    id: GLuint, 
    handles: BTreeMap<String, GLint> 
}

impl ShaderProgram {
    
    pub fn new(program_id: GLuint, resource_names: &[String]) -> Result<ShaderProgram, ShaderProgramError> {
        debug_assert!(program_id != 0);
        let mut program = Self {
            id: program_id,
            handles: BTreeMap::new()
        };

        program.use_program();

        for res_name in resource_names.iter() {
            program.load_handle(res_name)?;
        }

        //defaults texture array slot to 0 (only if resource texture array is loaded)
        match program.handles.get("texture_array") {
            Some(handle) => {
                unsafe { gl::Uniform1i(*handle, 0) }
                check_opengl_error("gl::Uniform1i")?;
            },
            _ => {}
        }

        Ok(program)
    }

    fn load_handle(&mut self, name: &str) -> Result<(), ShaderProgramError> {
        self.handles.insert(name.to_string(), get_resource_handle(self.id, name)?);
        Ok(())
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    pub fn set_resource_mat4(&self, resource_name: &str, matrix: &Matrix4<Float>) -> Result<(), ShaderProgramError> {
        match self.handles.get(resource_name) {
            Some(handle) => {
                unsafe { gl::UniformMatrix4fv(*handle, 1, gl::FALSE, matrix.as_array().as_ptr() as * const Float); }
                check_opengl_error("gl::UniformMatrix4fv")?;
                Ok(())
            },
            _ => { Err(ShaderProgramError::HandleNotExisting(resource_name.to_string())) }
        }
    }
    pub fn set_resource_vec3(&self, resource_name: &str, vector: &Vector3<Float>) -> Result<(), ShaderProgramError> {
        match self.handles.get(resource_name) {
            Some(handle) => {
                unsafe { gl::Uniform3fv(*handle, 1, vector.as_array().as_ptr() as * const Float); }
                check_opengl_error("gl::UniformMatrix4fv")?;
                Ok(())
            },
            _ => { Err(ShaderProgramError::HandleNotExisting(resource_name.to_string())) }
        }
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
    trace!("Getting resource handle for '{}'", resource_name);
    let res_name_zero_term = resource_name.to_string() + "\0";
    let handle: GLint = unsafe { gl::GetUniformLocation(program_id, res_name_zero_term.as_ptr() as *const _) };
    if handle == -1 {
        check_opengl_error("gl::GetUniformLocation")?;
        return Err(ShaderProgramError::FunctionFailure(format!("gl::GetUniformLocation('{}')", resource_name)));
    }
    trace!("Handle = {}", handle);
    Ok(handle)
}
