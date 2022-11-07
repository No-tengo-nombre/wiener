use gl;
use gl::types::*;

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
}
