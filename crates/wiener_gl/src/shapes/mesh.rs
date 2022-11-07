use crate::{VertexArray, VertexBuffer, ElementBuffer, ShaderProgram, Texture2D, Buffer};

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
}
