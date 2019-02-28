use std::ptr;
use gl;
use gl::types::{ GLuint, GLint, GLenum };

use crate::utility::read_file;
use crate::graphics::{ GraphicsError, check_opengl_error };
use super::{ ShaderProgram, ShaderError, ShaderProgramError };

pub struct ShaderProgramBuilder {
    shader_list: Vec<Shader>,
    resource_list: Vec<String>
}

struct Shader {
    shader_type: GLenum,
    shader_file_path: String,
}

struct BuildResources {
    program_id: GLuint,
    shader_ids: Vec<GLuint>
}

impl ShaderProgramBuilder {

    pub fn new() -> ShaderProgramBuilder {
        ShaderProgramBuilder {
            shader_list: Vec::new(),
            resource_list: Vec::new()
        }
    }

    //Also must add category in compile_shader() for new shader types!
    pub fn add_vertex_shader(self, shader_file_path: &str) -> Self {
        self.add_shader(gl::VERTEX_SHADER, shader_file_path)
    }

    pub fn add_fragment_shader(self, shader_file_path: &str) -> Self {
        self.add_shader(gl::FRAGMENT_SHADER, shader_file_path)
    }

    pub fn add_geometry_shader(self, shader_file_path: &str) -> Self {
        self.add_shader(gl::GEOMETRY_SHADER, shader_file_path)
    }

    pub fn add_resource(mut self, name: &str) -> Self {
        self.resource_list.push(name.to_string());
        self
    }

    fn add_shader(mut self, shader_type: GLenum, shader_file_path: &str) -> Self {
        let shader = Shader {
            shader_type: shader_type,
            shader_file_path: shader_file_path.to_string()
        };
        self.shader_list.push(shader);
        self
    }


    pub fn finish(self) -> Result<ShaderProgram, GraphicsError> {
        let resources = BuildResources::new(&self.shader_list)?;
        let program_id = resources.build()?;
        let program = ShaderProgram::new(program_id, self.resource_list.as_slice())?;
        Ok(program)
    }
}

impl BuildResources {
    pub fn new(shader_list: &[Shader]) -> Result<BuildResources, GraphicsError> {
        debug!("Creating new shader program");
        let shader_ids: Vec<GLuint> = compile_shaders(shader_list)?;
        let program_id: GLuint = unsafe { gl::CreateProgram() };
        if program_id == 0 {
            check_opengl_error("gl::CreateProgram")?;
            delete_shaders(&shader_ids);
            return Err(GraphicsError::from(ShaderProgramError::FunctionFailure("gl::CreateProgram".to_string())));
        }

       Ok(BuildResources {
            program_id: program_id,
            shader_ids: shader_ids
        })
    }

    pub fn build(mut self) -> Result<GLuint, GraphicsError> {
        self.attach_shaders()?;
        self.link_program()?;
        self.detach_shaders()?;
        self.delete_shaders()?;
        self.check_link_success()?;
        Ok(self.consume())
    }

    fn attach_shaders(&self) -> Result<(), GraphicsError> {
        trace!("Attaching shaders");
        for shader_id in &self.shader_ids {
            unsafe { gl::AttachShader(self.program_id, *shader_id); }
            check_opengl_error("gl::AttachShader")?;
        }
        Ok(())
    }

    fn link_program(&self) -> Result<(), GraphicsError> {
        debug!("Linking shader program");
        unsafe { gl::LinkProgram(self.program_id); }
        check_opengl_error("gl::LinkProgram")?;
        Ok(())
    }

    fn detach_shaders(&self) -> Result<(), GraphicsError> {
        trace!("Detaching shaders");
        for shader_id in &self.shader_ids {
            unsafe { gl::DetachShader(self.program_id, *shader_id); }
            check_opengl_error("gl::DetachShader")?;
        }
        Ok(())
    }

    fn delete_shaders(&mut self) -> Result<(), GraphicsError> {
        trace!("Deleting shaders");
        delete_shaders(&self.shader_ids);
        check_opengl_error("gl::DeleteShader")?;
        self.shader_ids.clear();
        Ok(())
    }

    fn check_link_success(&self) -> Result<(), GraphicsError> {
        let mut success: GLint = 0;
        unsafe { gl::GetProgramiv(self.program_id, gl::LINK_STATUS, &mut success); }
        check_opengl_error("gl::GetProgramiv")?;
        if success == 0 {
            return Err(GraphicsError::from(ShaderProgramError::Linkage(get_program_log(self.program_id))));
        }
        Ok(())
    }

    fn consume(mut self) -> GLuint {
        let id = self.program_id;
        self.program_id = 0;
        id
    }
}

impl Drop for BuildResources {
    fn drop(&mut self) {
        if self.program_id > 0 {
            detach_attached_shaders(self.program_id);
        }
        delete_shaders(&self.shader_ids);
        if self.program_id > 0 {
            delete_program(self.program_id);
        }
    }
}

fn compile_shaders(shader_list: &[Shader]) -> Result<Vec<GLuint>, ShaderError> {
    let mut shader_ids: Vec<GLuint> = Vec::new();
    for shader in shader_list {
        let shader_id = match compile_shader(shader) {
            Ok(s) => s,
            Err(e) => {
                    delete_shaders(&shader_ids);
                    return Err(e);
                }
        };
        shader_ids.push(shader_id);
    }
    Ok(shader_ids)
}

fn compile_shader(shader: &Shader) -> Result<GLuint, ShaderError> {
    let shader_name = match shader.shader_type {
        gl::FRAGMENT_SHADER => "fragment shader",
        gl::VERTEX_SHADER => "vertex shader",
        gl::GEOMETRY_SHADER => "geometry shader",
        unknown_type => { return Err(ShaderError::UnknownShaderType(unknown_type)); }
    };
    debug!("Compiling {}", shader_name);
    let source = read_file(&shader.shader_file_path)? + "\0";
    let shader_id = unsafe {
        let id = gl::CreateShader(shader.shader_type);
        gl::ShaderSource(id, 1, [source.as_ptr() as *const _].as_ptr(), ptr::null());
        gl::CompileShader(id);
        let mut success: GLint = 0;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let err = Err(ShaderError::Compilation(get_shader_log(id)));
            gl::DeleteShader(id);
            return err;
        }        
        id
    };
    Ok(shader_id)
}

fn delete_program(program_id: GLuint) {
    trace!("Deleting shader program");
    debug_assert!(program_id != 0);
    unsafe {
        gl::DeleteProgram(program_id);
    }
}

fn delete_shaders(shader_ids: &[GLuint]) {
    trace!("Deleting shaders");
    unsafe {
        for id in shader_ids {
            gl::DeleteShader(*id);
        }
    }
}

fn detach_attached_shaders(program_id: GLuint) {
    trace!("Detaching attached shaders");
    debug_assert!(program_id != 0);
    let attach_count = unsafe {
        let mut count: GLint = 0;
        gl::GetProgramiv(program_id, gl::ATTACHED_SHADERS, &mut count);
        count
    };
    let shader_ids = unsafe {
        let mut ids: Vec<GLuint> = Vec::with_capacity(attach_count as usize);
        ids.set_len(attach_count as usize);
        gl::GetAttachedShaders(program_id, attach_count, ptr::null_mut(), ids.as_ptr() as * mut _);
        ids 
    };
    unsafe {
        for id in shader_ids {
            gl::DetachShader(program_id, id);
        }
    }
}

fn get_shader_log(shader_id: GLuint) -> String {
    trace!("getting shader log for shader id = {}", shader_id);
    let mut log_len: GLint = 0;
    let mut bytes_written: GLint = 0;
    let mut log_vec: Vec<u8> = Vec::new();
    unsafe {
        gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut log_len);
        log_vec.reserve(log_len as usize);
        trace!("allocated log size: {}", log_vec.capacity()); 
        gl::GetShaderInfoLog(shader_id, log_vec.capacity() as i32, &mut bytes_written, log_vec.as_mut_ptr() as *mut _);
        log_vec.set_len(bytes_written as usize);
        trace!("log bytes written: {}", bytes_written);
    };
    match String::from_utf8(log_vec) {
        Ok(ref s) if s.len() == 0 => "EMPTY_LOG".to_string(),
        Ok(s) => s,
        Err(_) => "couldn't convert shader log".to_string()
    }
}

fn get_program_log(program_id: GLuint) -> String {
    trace!("getting program log");
    let mut log_len: GLint = 0;
    let mut bytes_written: GLint = 0;
    let mut log_vec: Vec<u8> = Vec::new();
    unsafe {
        gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut log_len);
        log_vec.reserve(log_len as usize);
        trace!("allocated log size: {}", log_vec.capacity()); 
        gl::GetProgramInfoLog(program_id, log_vec.capacity() as i32, &mut bytes_written, log_vec.as_mut_ptr() as *mut _);
        log_vec.set_len(bytes_written as usize);
        trace!("log bytes written: {}", bytes_written);
    };
    match String::from_utf8(log_vec) {
        Ok(ref s) if s.len() == 0 => "EMPTY_LOG".to_string(),
        Ok(s) => s,
        Err(_) => "couldn't convert shader program log".to_string()
    }
}
