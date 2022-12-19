use super::buffer::Buffer;
use std::mem::size_of;

use gl;
use gl::types::*;

/// Uniform buffer object, which contains uniform data stored in the GPU.
#[derive(Copy, Clone, Debug)]
pub struct UniformBuffer {
    /// Unique ID associated to the object.
    _id: u32,

    /// Size of the data.
    pub _size: u32,

    /// Usage of the data.
    pub _usage: GLenum,
}

impl UniformBuffer {
    /// Generate a new uniform buffer.
    pub fn new(size: u32) -> Self {
        let mut ubo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ubo_id);
        }

        return UniformBuffer {
            _id: ubo_id,
            _size: size,
            _usage: gl::STATIC_DRAW,
        };
    }

    /// Set the usage of the uniform buffer.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        self._usage = new_usage;
        return self;
    }

    /// Binds the uniform buffer to the given index in memory.
    pub fn bind_index(&self, index: u32) {
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, self._id);
        }
    }
}

impl Buffer for UniformBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self._id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }

    fn buffer_data<T>(&self, data: &[T]) -> Self {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const GLvoid,
                self._usage,
            );
        }
        return *self;
    }
}
