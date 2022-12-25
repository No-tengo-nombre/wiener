use crate::{Bindable, Buffer};
use std::mem::size_of;

use gl;
use gl::types::*;

/// Vertex buffer object, which contains vertex data stored in the GPU.
#[derive(Copy, Clone, Debug)]
pub struct VertexBuffer {
    /// Unique ID associated to the object.
    _id: u32,

    /// Usage of the data.
    pub _usage: GLenum,
}

impl VertexBuffer {
    /// Generate a new vertex buffer.
    pub fn new() -> Self {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
        }

        return VertexBuffer {
            _id: vbo_id,
            _usage: gl::STATIC_DRAW,
        };
    }

    /// Set the usage of the vertex buffer.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        self._usage = new_usage;
        return self;
    }

    /// Set the usage of the vertex buffer.
    pub fn set_usage(&mut self, new_usage: GLenum) {
        self._usage = new_usage;
    }
}

impl Bindable for VertexBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self._id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }
}

impl Buffer for VertexBuffer {
    fn buffer_data<T>(&self, data: &[T]) -> Self {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const GLvoid,
                self._usage,
            );
        }
        return *self;
    }
}
