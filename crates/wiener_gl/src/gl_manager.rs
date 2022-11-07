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
}
