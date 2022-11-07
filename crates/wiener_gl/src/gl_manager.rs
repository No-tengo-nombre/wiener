use gl;
use gl::types::*;

// TODO: Figure out a better way to do all of this.

/// Manager for OpenGL functions.
pub struct GLManager;

impl GLManager {
    pub fn enable(feature: GLenum) {
        unsafe {
            gl::Enable(feature);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear(mask: u32) {
        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn blend_func(sfactor: u32, dfactor: u32) {
        unsafe {
            gl::BlendFunc(sfactor, dfactor);
        }
    }
}
