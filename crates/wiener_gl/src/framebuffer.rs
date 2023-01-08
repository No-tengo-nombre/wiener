use crate::{Bindable, HasID, RenderBuffer, Texture, Texture2D};

use gl;
use gl::types::*;

/// OpenGL framebuffer.
#[derive(Clone, Debug)]
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

    /// Attach a depth and stencil texture without returning.
    pub fn inplace_attach_depth_stencil(&self, target: &dyn Texture) {
        self.inplace_attach_texture(gl::DEPTH_STENCIL_ATTACHMENT, target);
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

    /// Attach a multisampled 2D texture without returning.
    pub fn inplace_attach_multisampled_texture2d(&self, attachment_num: u32, target: &Texture2D) {
        self.inplace_attach_raw_texture2d(
            gl::COLOR_ATTACHMENT0 + attachment_num,
            gl::TEXTURE_2D_MULTISAMPLE,
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

    /// Attach a depth and stencil texture, returning `self` afterwards.
    pub fn attach_depth_stencil(self, target: &dyn Texture) -> Self {
        self.inplace_attach_depth_stencil(target);
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

    /// Attach a multisampled 2D texture, returning `self` afterwards.
    pub fn attach_multisampled_texture2d(self, attachment_num: u32, target: &Texture2D) -> Self {
        self.inplace_attach_multisampled_texture2d(attachment_num, target);
        return self;
    }

    /// Bind to the read framebuffer.
    pub fn bind_read(&self) {
        log::trace!("FrameBuffer :: Binding to the read framebuffer");
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.get_id());
        }
    }

    /// Bind to the draw framebuffer.
    pub fn bind_draw(&self) {
        log::trace!("FrameBuffer :: Binding to the draw framebuffer");
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.get_id());
        }
    }

    /// Blit the framebuffer's texture to another framebuffer.
    pub fn blit(
        &self,
        target: &FrameBuffer,
        source_coords: (i32, i32, i32, i32),
        target_coords: (i32, i32, i32, i32),
        mask: GLenum,
        filter: GLenum,
    ) {
        log::trace!("FrameBuffer :: Blitting to another framebuffer");
        self.bind_read();
        target.bind_draw();
        unsafe {
            gl::BlitFramebuffer(
                source_coords.0,
                source_coords.1,
                source_coords.2,
                source_coords.3,
                target_coords.0,
                target_coords.1,
                target_coords.2,
                target_coords.3,
                mask,
                filter,
            );
        }
        self.unbind();
    }
}

impl Bindable for FrameBuffer {
    fn bind(&self) {
        log::trace!("FrameBuffer :: Binding");
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.get_id());
        }
    }

    fn unbind(&self) {
        log::trace!("FrameBuffer :: Unbinding");
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    fn delete(&self) {
        log::trace!("FrameBuffer :: Deleting");
        unsafe {
            gl::DeleteFramebuffers(1, &self.get_id());
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        self.delete();
    }
}
