use crate::{Bindable, HasID, RenderBuffer, Texture, Texture2D};

use gl;
use gl::types::*;

/// OpenGL framebuffer.
#[derive(Clone, Copy, Debug)]
pub struct FrameBuffer {
    _id: u32,
}

impl HasID for FrameBuffer {
    fn get_id(&self) -> u32 {
        return self._id;
    }
}

impl FrameBuffer {
    /// Create a new framebuffer.
    pub fn new() -> Self {
        let mut fbo_id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo_id);
        }
        return FrameBuffer { _id: fbo_id };
    }

    /// Verify the validity of the framebuffer.
    pub fn verify(&self) {
        self.bind();
        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("The framebuffer is not complete");
            }
        }
        self.unbind();
    }

    /// Attach a renderbuffer without returning.
    pub fn inplace_attach_renderbuffer(&self, attachment: GLenum, target: &RenderBuffer) {
        self.bind();
        unsafe {
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                attachment,
                gl::RENDERBUFFER,
                target.get_id(),
            );
        }
        self.unbind();
    }

    /// Attach a texture without returning.
    pub fn inplace_attach_texture(&self, attachment: GLenum, target: &dyn Texture) {
        self.bind();
        unsafe {
            gl::FramebufferTexture(gl::FRAMEBUFFER, attachment, target.get_id(), 0);
        }
        self.unbind();
    }

    /// Attach a depth texture without returning.
    pub fn inplace_attach_depth(&self, target: &dyn Texture) {
        self.inplace_attach_texture(gl::DEPTH_ATTACHMENT, target);
    }

    /// Attach an arbitrary 2D texture without returning.
    pub fn inplace_attach_raw_texture2d(
        &self,
        attachment: GLenum,
        target_type: GLenum,
        target: &Texture2D,
    ) {
        self.bind();
        unsafe {
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, target_type, target.get_id(), 0);
        }
        self.unbind();
    }

    /// Attach a 2D texture without returning.
    pub fn inplace_attach_texture2d(&self, attachment_num: u32, target: &Texture2D) {
        self.inplace_attach_raw_texture2d(
            gl::COLOR_ATTACHMENT0 + attachment_num,
            gl::TEXTURE_2D,
            target,
        );
    }

    /// Attach a renderbuffer, returning `self` afterwards.
    pub fn attach_renderbuffer(self, attachment: GLenum, target: &RenderBuffer) -> Self {
        self.inplace_attach_renderbuffer(attachment, target);
        return self;
    }

    /// Attach a texture, returning `self` afterwards.
    pub fn attach_texture(self, attachment: GLenum, target: &dyn Texture) -> Self {
        self.inplace_attach_texture(attachment, target);
        return self;
    }

    /// Attach a depth texture, returning `self` afterwards.
    pub fn attach_depth(self, target: &dyn Texture) -> Self {
        self.inplace_attach_depth(target);
        return self;
    }

    /// Attach an arbitrary 2D texture, returning `self` afterwards.
    pub fn attach_raw_texture2d(
        self,
        attachment: GLenum,
        target_type: GLenum,
        target: &Texture2D,
    ) -> Self {
        self.inplace_attach_raw_texture2d(attachment, target_type, target);
        return self;
    }

    /// Attach a 2D texture, returning `self` afterwards.
    pub fn attach_texture2d(self, attachment_num: u32, target: &Texture2D) -> Self {
        self.inplace_attach_texture2d(attachment_num, target);
        return self;
    }

    pub fn bind_read(&self) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.get_id());
        }
    }

    pub fn bind_draw(&self) {
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.get_id());
        }
    }
}

impl Bindable for FrameBuffer {
    fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    fn delete(&self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.get_id());
        }
    }
}
