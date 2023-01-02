use crate::{Bindable, HasID, Texture};

use gl;
use gl::types::*;

/// OpenGL render buffer, which can be drawn to efficiently, but is not
/// intended for reading.
#[derive(Clone, Copy, Debug)]
pub struct RenderBuffer {
    _id: u32,
}

impl Texture for RenderBuffer {}

impl HasID for RenderBuffer {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl RenderBuffer {
    /// Create a new render buffer.
    pub fn new() -> Self {
        let mut rbo = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut rbo);
        }
        log::info!("RenderBuffer :: Creating renderbuffer {:?}", rbo);
        return RenderBuffer { _id: rbo };
    }

    /// Set up the render buffer, giving it a format, width and height.
    pub fn set_up(self, format: GLenum, width: i32, height: i32) -> Self {
        log::debug!("RenderBuffer :: Setting up render buffer");
        self.bind();
        unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, format, width, height);
        }
        self.unbind();
        return self;
    }

    /// Set up a multisampled renderbuffer.
    pub fn set_up_multisample(self, samples: i32, format: GLenum, width: i32, height: i32) -> Self {
        log::debug!("RenderBuffer :: Setting up multisampled render buffer");
        self.bind();
        unsafe {
            gl::RenderbufferStorageMultisample(gl::RENDERBUFFER, samples, format, width, height);
        }
        self.unbind();
        return self;
    }
}

impl Bindable for RenderBuffer {
    fn bind(&self) {
        log::trace!("RenderBuffer :: Binding");
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!("RenderBuffer :: Unbinding");
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }

    fn delete(&self) {
        log::trace!("RenderBuffer :: Deleting");
        unsafe {
            gl::DeleteRenderbuffers(1, &self.get_id());
        }
    }
}
