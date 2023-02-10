use crate::{Bindable, Buffer, HasID};
use std::mem::size_of;

use gl;
use gl::types::*;

/// Vertex buffer object, which contains vertex data stored in the GPU.
#[derive(Clone, Copy, Debug)]
pub struct VertexBuffer {
    /// Unique ID associated to the object.
    _id: u32,

    /// Usage of the data.
    pub usage: GLenum,
}

impl HasID for VertexBuffer {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl VertexBuffer {
    /// Generate a new vertex buffer.
    pub fn new() -> Self {
        let mut vbo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_id);
        }
        log::info!("VertexBuffer :: Creating new VertexBuffer {:?}", vbo_id);

        return VertexBuffer {
            _id: vbo_id,
            usage: gl::STATIC_DRAW,
        };
    }

    /// Set the usage of the vertex buffer.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        log::trace!("VertexBuffer :: Setting usage");
        self.usage = new_usage;
        return self;
    }
}

impl Bindable for VertexBuffer {
    fn bind(&self) {
        log::trace!("VertexBuffer :: Binding");
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!("VertexBuffer :: Unbinding");
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        log::info!("VertexBuffer :: Deleting");
        unsafe {
            gl::DeleteBuffers(1, &self.get_id());
        }
    }
}

impl Buffer for VertexBuffer {
    fn buffer_data<T>(&self, data: &[T]) {
        let size = data.len() * size_of::<T>();
        log::info!("VertexBuffer :: Buffering {:?} bytes to GPU", size);
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size as isize,
                data.as_ptr() as *const GLvoid,
                self.usage,
            );
        };
    }
}
