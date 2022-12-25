use crate::{Bindable, Buffer};
use std::mem::size_of;

use gl;
use gl::types::*;

/// Element buffer object, which contains triangle data stored in the GPU.
#[derive(Copy, Clone, Debug)]
pub struct ElementBuffer {
    /// Unique ID associated to the object.
    _id: u32,

    /// Usage of the data.
    pub _usage: GLenum,
}

impl ElementBuffer {
    /// Generate a new vertex buffer.
    pub fn new() -> Self {
        let mut ebo_id = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo_id);
        }

        return ElementBuffer {
            _id: ebo_id,
            _usage: gl::STATIC_DRAW,
        };
    }

    /// Set the usage of the element buffer.
    pub fn usage(mut self, new_usage: GLenum) -> Self {
        self._usage = new_usage;
        return self;
    }

    /// Set the usage of the element buffer.
    pub fn set_usage(&mut self, new_usage: GLenum) {
        self._usage = new_usage;
    }
}

impl Bindable for ElementBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self._id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self._id);
        }
    }
}

impl Buffer for ElementBuffer {
    fn buffer_data<T>(&self, data: &[T]) -> Self {
        self.bind();
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * size_of::<T>()) as isize,
                data.as_ptr() as *const GLvoid,
                self._usage,
            );
        }
        return *self;
    }
}
