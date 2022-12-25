use std::ffi::c_void;

use crate::{
    Bindable, Buffer, Drawable, ElementBuffer, ShaderProgram, Texture2D, VertexArray, VertexBuffer,
};

use gl::types::GLenum;
use log::{info, trace};
use wiener_utils::math;

pub struct Mesh {
    pub vao: VertexArray,
    pub vbo: VertexBuffer,
    pub ebo: ElementBuffer,
    pub primitive: GLenum,
    _primitive_num: i32,
    pub shader: ShaderProgram,
    pub textures: Vec<Texture2D>,
    pub model_mat: [[f32; 4]; 4],
    pub view_mat: [[f32; 4]; 4],
    pub projection_mat: [[f32; 4]; 4],
}

impl Mesh {
    pub fn new() -> Self {
        info!("Mesh :: Creating mesh");
        let vao = VertexArray::builder();
        vao.bind();
        return Mesh {
            vao,
            vbo: VertexBuffer::new(),
            ebo: ElementBuffer::new(),
            primitive: gl::TRIANGLES,
            _primitive_num: 0,
            shader: ShaderProgram::new(),
            textures: [].to_vec(),
            model_mat: math::linalg::eye4::<f32>(),
            view_mat: math::linalg::eye4::<f32>(),
            projection_mat: math::linalg::eye4::<f32>(),
        };
    }

    pub fn vertices<T>(mut self, new_vertices: &[T]) -> Self {
        self.set_vertices(new_vertices);
        return self;
    }

    pub fn indices<T>(mut self, new_indices: &[T]) -> Self {
        self.set_indices(new_indices);
        return self;
    }

    pub fn shader(mut self, new_shader: ShaderProgram) -> Self {
        trace!("Mesh :: Setting shader");
        self.shader = new_shader;
        return self;
    }

    pub fn textures(mut self, new_textures: Vec<Texture2D>) -> Self {
        trace!("Mesh :: Setting textures");
        self.textures = new_textures;
        return self;
    }

    pub fn add_texture(&mut self, texture: Texture2D) {
        trace!("Mesh :: Adding texture");
        self.textures.push(texture);
    }

    pub fn usage(mut self, new_usage: GLenum) -> Self {
        trace!("Mesh :: Setting usage");
        self.vbo.usage = new_usage;
        return self;
    }

    pub fn layout(mut self, new_layout: &[u32]) -> Self {
        trace!("Mesh :: Setting layout");
        self.vao.layout = new_layout.to_vec();
        return self;
    }

    pub fn primitive(mut self, new_primitive: GLenum) -> Self {
        trace!("Mesh :: Setting primitive type");
        self.primitive = new_primitive;
        return self;
    }

    pub fn set_vertices<T>(&mut self, new_vertices: &[T]) {
        trace!("Mesh :: Setting vertices");
        self.vbo.buffer_data(new_vertices);
        let size = std::mem::size_of::<T>();
        info!("Mesh :: Setting associated VAO size to {:?}", size);
        self.vao.size = size as u32;
    }

    pub fn set_indices<T>(&mut self, new_indices: &[T]) {
        trace!("Mesh :: Setting indices");
        self.ebo.buffer_data(new_indices);
        self._primitive_num = new_indices.len() as i32;
    }

    pub fn set_usage(&mut self, new_usage: GLenum) {
        trace!("Mesh :: Setting usage");
        self.vbo.usage = new_usage;
        self.ebo.usage = new_usage;
    }

    pub fn set_layout(&mut self, new_layout: &[u32]) {
        trace!("Mesh :: Setting layout");
        self.vao.layout = new_layout.to_vec();
    }

    pub fn model_mat(mut self, new_model_mat: [[f32; 4]; 4]) -> Self {
        trace!("Mesh :: Setting model matrix");
        self.model_mat = new_model_mat;
        return self;
    }

    pub fn view_mat(mut self, new_view_mat: [[f32; 4]; 4]) -> Self {
        trace!("Mesh :: Setting view matrix");
        self.view_mat = new_view_mat;
        return self;
    }

    pub fn projection_mat(mut self, new_projection_mat: [[f32; 4]; 4]) -> Self {
        trace!("Mesh :: Setting projection matrix");
        self.projection_mat = new_projection_mat;
        return self;
    }

    pub fn set_model_mat(mut self, new_model_mat: [[f32; 4]; 4]) {
        trace!("Mesh :: Setting model matrix");
        self.model_mat = new_model_mat;
    }

    pub fn set_view_mat(mut self, new_view_mat: [[f32; 4]; 4]) {
        trace!("Mesh :: Setting view matrix");
        self.view_mat = new_view_mat;
    }

    pub fn set_projection_mat(mut self, new_projection_mat: [[f32; 4]; 4]) {
        trace!("Mesh :: Setting projection matrix");
        self.projection_mat = new_projection_mat;
    }
}

impl Bindable for Mesh {
    fn bind(&self) {
        trace!("Mesh :: Binding");
        self.vao.bind();
        self.vbo.bind();
        self.ebo.bind();
        self.shader.bind();
        for t in &self.textures {
            t.bind();
        }
    }

    fn unbind(&self) {
        trace!("Mesh :: Unbinding");
        self.vao.unbind();
        self.vbo.unbind();
        self.ebo.unbind();
        self.shader.unbind();
        for t in &self.textures {
            t.unbind();
        }
    }

    fn delete(&self) {
        trace!("Mesh :: Deleting");
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
        self.shader.delete();
        for t in &self.textures {
            t.delete();
        }
    }
}

impl Drawable for Mesh {
    fn draw(&self) {
        trace!("Mesh :: Sending draw call");
        self.bind();

        // Uniform the MVP matrices
        self.shader.uniform_mat4f("u_model", self.model_mat);
        self.shader.uniform_mat4f("u_view", self.view_mat);
        self.shader
            .uniform_mat4f("u_projection", self.projection_mat);

        unsafe {
            gl::DrawElements(
                self.primitive,
                self._primitive_num,
                gl::UNSIGNED_INT,
                0 as *const c_void,
            );
        }
    }
}
