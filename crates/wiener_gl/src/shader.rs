use crate::Bindable;

use gl;
use gl::types::*;
use log;
use std::collections::HashMap;
use std::fs;

fn get_shader_type(file_extension: &str) -> GLenum {
    let file_map = HashMap::from([
        // Vertex shaders
        ("v", gl::VERTEX_SHADER),
        ("vs", gl::VERTEX_SHADER),
        ("vsh", gl::VERTEX_SHADER),
        ("vert", gl::VERTEX_SHADER),
        ("vertex", gl::VERTEX_SHADER),
        // Fragment shaders
        ("f", gl::FRAGMENT_SHADER),
        ("fs", gl::FRAGMENT_SHADER),
        ("fsh", gl::FRAGMENT_SHADER),
        ("frag", gl::FRAGMENT_SHADER),
        ("fragment", gl::FRAGMENT_SHADER),
        // Geometry shaders
        ("g", gl::GEOMETRY_SHADER),
        ("gs", gl::GEOMETRY_SHADER),
        ("geom", gl::GEOMETRY_SHADER),
        ("geometry", gl::GEOMETRY_SHADER),
        // Tessellation shaders
        ("control", gl::TESS_CONTROL_SHADER),
        ("tesc", gl::TESS_CONTROL_SHADER),
        ("tescontrol", gl::TESS_CONTROL_SHADER),
        ("tesscontrol", gl::TESS_CONTROL_SHADER),
        ("eval", gl::TESS_EVALUATION_SHADER),
        ("tese", gl::TESS_EVALUATION_SHADER),
        ("teseval", gl::TESS_EVALUATION_SHADER),
        ("tesseval", gl::TESS_EVALUATION_SHADER),
        // Compute shaders
        ("comp", gl::COMPUTE_SHADER),
        ("compute", gl::COMPUTE_SHADER),
    ]);
    return file_map[file_extension];
}

/// OpenGL shader component.
#[derive(Copy, Clone, Debug)]
pub struct Shader {
    _id: u32,

    /// Type of shader.
    _type: GLenum,
}

/// Program that contains a bunch of compiled shaders.
#[derive(Clone, Debug)]
pub struct ShaderProgram<'a> {
    _id: u32,
    _shaders: &'a [Shader],
}

impl Shader {
    /// Create a new shader.
    pub fn new(content: &str, shader_type: GLenum) -> Self {
        let shader_id;
        let mut success = 0;
        unsafe {
            // Compile the shader
            shader_id = gl::CreateShader(shader_type);
            log::info!("Shader :: Creating new shader {:?}", shader_id);
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

    /// Create a new shader from a file, assuming the shader type from the file extension.
    pub fn from_file(filename: &str) -> Self {
        let shader_content = fs::read_to_string(filename)
            .expect(format!("Error reading file {:?}.", filename).as_str());
        let file_extension = filename
            .split(".")
            .last()
            .expect("Couldn't find file extension.");
        return Shader::new(&shader_content, get_shader_type(file_extension));
    }

    /// Create a new shader from a file, explicitly giving the shader type.
    pub fn from_file_explicit(filename: &str, shader_type: GLenum) -> Self {
        let shader_content = fs::read_to_string(filename)
            .expect(format!("Error reading file {:?}.", filename).as_str());
        return Shader::new(&shader_content, shader_type);
    }

    /// Delete the shader.
    pub fn delete(self) {
        log::info!("Shader :: Deleting shader");
        unsafe {
            gl::DeleteShader(self._id);
        }
    }
}

impl<'a> ShaderProgram<'a> {
    pub fn new() -> Self {
        unsafe {
            let program_id = gl::CreateProgram();
            log::info!(
                "ShaderProgram :: Creating new shader program {:?}",
                program_id
            );
            return ShaderProgram {
                _id: program_id,
                _shaders: &[],
            };
        }
    }

    pub fn from_array(shaders: &'a [Shader]) -> Self {
        return Self::new().shaders(shaders);
    }

    pub fn shaders(mut self, shaders: &'a [Shader]) -> Self {
        log::info!("ShaderProgram :: Setting shaders");
        self._shaders = shaders;

        for shader in self._shaders {
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
        }
        return self;
    }

    pub fn get_uniform_location(&self, name: &str) -> GLint {
        unsafe {
            // Strings in rust are not null terminated, so we terminate them manually.
            return gl::GetUniformLocation(self._id, format!("{name}\0").as_ptr() as *const GLchar);
        }
    }

    pub fn uniform_1i(&self, name: &str, val0: i32) {
        self.bind();
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), val0);
        }
    }

    pub fn uniform_2i(&self, name: &str, val0: i32, val1: i32) {
        self.bind();
        unsafe {
            gl::Uniform2i(self.get_uniform_location(name), val0, val1);
        }
    }

    pub fn uniform_3i(&self, name: &str, val0: i32, val1: i32, val2: i32) {
        self.bind();
        unsafe {
            gl::Uniform3i(self.get_uniform_location(name), val0, val1, val2);
        }
    }

    pub fn uniform_4i(&self, name: &str, val0: i32, val1: i32, val2: i32, val3: i32) {
        self.bind();
        unsafe {
            gl::Uniform4i(self.get_uniform_location(name), val0, val1, val2, val3);
        }
    }

    pub fn uniform_1f(&self, name: &str, val0: f32) {
        self.bind();
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), val0);
        }
    }

    pub fn uniform_2f(&self, name: &str, val0: f32, val1: f32) {
        self.bind();
        unsafe {
            gl::Uniform2f(self.get_uniform_location(name), val0, val1);
        }
    }

    pub fn uniform_3f(&self, name: &str, val0: f32, val1: f32, val2: f32) {
        self.bind();
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), val0, val1, val2);
        }
    }

    pub fn uniform_4f(&self, name: &str, val0: f32, val1: f32, val2: f32, val3: f32) {
        self.bind();
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), val0, val1, val2, val3);
        }
    }

    pub fn uniform_mat2f(&self, name: &str, val: [[f32; 2]; 2]) {
        self.bind();
        unsafe {
            gl::UniformMatrix2fv(
                self.get_uniform_location(name),
                1,
                gl::TRUE,
                val[0].as_ptr(),
            )
        }
    }

    pub fn uniform_mat3f(&self, name: &str, val: [[f32; 3]; 3]) {
        self.bind();
        unsafe {
            gl::UniformMatrix3fv(
                self.get_uniform_location(name),
                1,
                gl::TRUE,
                val[0].as_ptr(),
            )
        }
    }

    pub fn uniform_mat4f(&self, name: &str, val: [[f32; 4]; 4]) {
        self.bind();
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(name),
                1,
                gl::TRUE,
                val[0].as_ptr(),
            )
        }
    }
}

impl<'a> Bindable for ShaderProgram<'a> {
    fn bind(&self) {
        log::trace!("ShaderProgram :: Binding");
        unsafe {
            gl::UseProgram(self._id);
        }
    }

    fn unbind(&self) {
        log::trace!("ShaderProgram :: Unbinding");
        unsafe {
            gl::UseProgram(self._id);
        }
    }

    fn delete(&self) {
        log::info!("ShaderProgram :: Deleting");
        unsafe {
            gl::DeleteProgram(self._id);
        }
    }
}
