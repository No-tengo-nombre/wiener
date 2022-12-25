use gl;
use gl::types::*;
use log;

// TODO: Figure out a better way to do all of this.

/// Manager for OpenGL functions.
pub struct GLManager;

impl GLManager {
    pub fn enable(feature: GLenum) {
        log::info!("GLManager :: Enabling feature {:?}", feature);
        unsafe {
            gl::Enable(feature);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        log::info!("GLManager :: Setting clear color");
        unsafe {
            gl::ClearColor(red, green, blue, alpha);
        }
    }

    pub fn clear(mask: u32) {
        log::trace!("GLManager :: Clearing screen");
        unsafe {
            gl::Clear(mask);
        }
    }

    pub fn blend_func(sfactor: u32, dfactor: u32) {
        log::info!("GLManager :: Setting blending function");
        unsafe {
            gl::BlendFunc(sfactor, dfactor);
        }
    }
}
