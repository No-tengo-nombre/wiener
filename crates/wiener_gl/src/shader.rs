use gl;
use gl::types::*;
use std::fs;

/// OpenGL shader component.
#[derive(Copy, Clone, Debug)]
pub struct Shader {
    _id: u32,

    /// Type of shader.
    pub _type: GLenum,
}

/// Program that contains a bunch of compiled shaders.
#[derive(Clone, Debug)]
pub struct ShaderProgram {
    _id: u32,
    _shaders: Vec<Shader>,
}

impl Shader {
    /// Create a new shader.
    pub fn new(content: &str, shader_type: GLenum) -> Self {
        let shader_id;
        let mut success = 0;
        unsafe {
            // Compile the shader
            shader_id = gl::CreateShader(shader_type);
            gl::ShaderSource(
                shader_id,
                1,
                &(content.as_bytes().as_ptr().cast()),
                &(content.len().try_into().unwrap()),
            );
            gl::CompileShader(shader_id);

            // Verify compilation
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(shader_id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Failed to compile shader : {}", String::from_utf8_lossy(&v));
            }
        }
        return Shader {
            _id: shader_id,
            _type: shader_type,
        };
    }

    /// Get the shader ID.
    pub fn get_id(&self) -> u32 {
        return self._id;
    }

    /// Create a new shader from a file.
    pub fn from_file(filename: &str, shader_type: GLenum) -> Self {
        let shader_content = fs::read_to_string(filename).expect("Error reading shader code.");
        return Shader::new(&shader_content, shader_type);
    }

    /// Delete the shader.
    pub fn delete(self) {
        unsafe {
            gl::DeleteShader(self._id);
        }
    }
}

impl ShaderProgram {
    pub fn new() -> Self {
        unsafe {
            return ShaderProgram {
                _id: gl::CreateProgram(),
                _shaders: [].to_vec(),
            };
        }
    }

    pub fn add_shader(mut self, shader: Shader) -> Self {
        self._shaders.push(shader);

        unsafe {
            gl::AttachShader(self._id, shader.get_id());
            gl::LinkProgram(self._id);
            let mut success = 0;
            gl::GetProgramiv(self._id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(self._id, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            };
        }
        shader.delete();
        return self;
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self._id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(self._id);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self._id);
        }
    }
}
