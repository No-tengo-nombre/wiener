use gl;
use gl::types::*;
use log;

// TODO: Figure out a better way to do all of this.

/// Manager for OpenGL functions.
pub struct GLManager;

impl GLManager {
    pub fn enable(feature: GLenum) {
        log::trace!("GLManager :: Enabling feature {:?}", feature);
        unsafe {
            gl::Enable(feature);
        }
    }

    pub fn disable(feature: GLenum) {
        log::trace!("GLManager :: Disabling feature {:?}", feature);
        unsafe {
            gl::Disable(feature);
        }
    }

    pub fn clear_color(red: f32, green: f32, blue: f32, alpha: f32) {
        log::trace!("GLManager :: Setting clear color");
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
        log::trace!("GLManager :: Setting blending function");
        unsafe {
            gl::BlendFunc(sfactor, dfactor);
        }
    }

    pub fn viewport(start_x: i32, start_y: i32, width: i32, height: i32) {
        log::trace!(
            "GLManager :: Setting viewport to {:?}x{:?} at ({:?}, {:?})",
            width,
            height,
            start_x,
            start_y
        );
        unsafe {
            gl::Viewport(start_x, start_y, width, height);
        }
    }

    pub fn polygon_mode(face: GLenum, mode: GLenum) {
        log::trace!(
            "GLManager :: Setting polygon mode on face {:?} to {:?}",
            face,
            mode
        );
        unsafe {
            gl::PolygonMode(face, mode);
        }
    }
}
