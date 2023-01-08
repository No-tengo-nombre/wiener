use crate::{Bindable, Buffer, HasID};
use std::mem::size_of;

use gl;
use gl::types::*;
use log;

/// Uniform buffer object, which contains uniform data stored in the GPU.
#[derive(Clone, Copy, Debug)]
pub struct UniformBuffer {
    /// Unique ID associated to the object.
    _id: u32,

    /// Size of the data.
    pub size: u32,

    /// Usage of the data.
    pub usage: GLenum,
}

impl HasID for UniformBuffer {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl UniformBuffer {
    /// Generate a new uniform buffer.
    pub fn new(size: u32) -> Self {
        let mut ubo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ubo_id);
        }
        log::info!("UniformBuffer :: Creating new UniformBuffer {:?}", ubo_id);

        return UniformBuffer {
            _id: ubo_id,
            size,
            usage: gl::STATIC_DRAW,
        };
    }

    /// Set the usage of the uniform buffer.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        log::trace!("UniformBuffer :: Setting usage");
        self.usage = new_usage;
        return self;
    }

    /// Binds the uniform buffer to the given index in memory.
    pub fn bind_index(&self, index: u32) {
        log::trace!("UniformBuffer :: Binding index");
        unsafe {
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, self.get_id());
        }
    }
}

impl Bindable for UniformBuffer {
    fn bind(&self) {
        log::trace!("UniformBuffer :: Binding");
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!("UniformBuffer :: Unbinding");
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    fn delete(&self) {
        log::info!("UniformBuffer :: Deleting");
        unsafe {
            gl::DeleteBuffers(1, &self.get_id());
        }
    }
}

impl Buffer for UniformBuffer {
    fn buffer_data<T>(&self, data: &[T]) {
        log::info!("UniformBuffer :: Buffering data to GPU");
        self.bind();
        unsafe {
            gl::BufferData(
                gl::UNIFORM_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const GLvoid,
                self.usage,
            );
        };
    }
}
