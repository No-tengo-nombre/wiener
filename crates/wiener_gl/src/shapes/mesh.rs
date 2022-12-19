use crate::{Buffer, ElementBuffer, ShaderProgram, Texture2D, VertexArray, VertexBuffer};

pub struct Mesh {
    _vao: VertexArray,
    _vbo: VertexBuffer,
    _ebo: ElementBuffer,
    _shader: ShaderProgram,
    _textures: Vec<Texture2D>,
}

impl Mesh {
    pub fn builder() -> Self {
        return Mesh {
            _vao: VertexArray::builder(),
            _vbo: VertexBuffer::new(),
            _ebo: ElementBuffer::new(),
            _shader: ShaderProgram::new(),
            _textures: [].to_vec(),
        };
    }

    pub fn vertices<T>(self, new_vertices: &[T]) -> Self {
        self._vbo.buffer_data(new_vertices);
        return self;
    }

    pub fn indices<T>(self, new_indices: &[T]) -> Self {
        self._ebo.buffer_data(new_indices);
        return self;
    }

    pub fn vao(mut self, new_vao: VertexArray) -> Self {
        self._vao = new_vao;
        return self;
    }

    pub fn vbo(mut self, new_vbo: VertexBuffer) -> Self {
        self._vbo = new_vbo;
        return self;
    }

    pub fn ebo(mut self, new_ebo: ElementBuffer) -> Self {
        self._ebo = new_ebo;
        return self;
    }

    pub fn shader(mut self, new_shader: ShaderProgram) -> Self {
        self._shader = new_shader;
        return self;
    }

    pub fn textures(mut self, new_textures: Vec<Texture2D>) -> Self {
        self._textures = new_textures;
        return self;
    }

    pub fn add_texture(&mut self, texture: Texture2D) {
        self._textures.push(texture);
    }
}
