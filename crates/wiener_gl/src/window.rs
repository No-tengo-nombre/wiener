use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;
use wiener_core::{init_glfw, WindowDescriptor};

/// Window that can render OpenGL stuff.
pub struct GLWindow {
    _descriptor: WindowDescriptor,
    _glfw_window: glfw::Window,
    _events: Receiver<(f64, glfw::WindowEvent)>,
    _glfw_instance: glfw::Glfw,
    _gl_version: (u32, u32),
    _gl_profile: glfw::OpenGlProfileHint,
}

impl GLWindow {
    /// Generate a builder for the window.
    pub fn builder() -> Self {
        let mut empty_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, receiver) = empty_instance
            .create_window(1, 1, "", glfw::WindowMode::Windowed)
            .expect("");
        return GLWindow {
            _descriptor: WindowDescriptor::default(),
            _glfw_window: window,
            _events: receiver,
            _glfw_instance: empty_instance,
            _gl_version: (4, 6),
            _gl_profile: glfw::OpenGlProfileHint::Core,
        };
    }

    /// Get the GLFW window.
    pub fn get_window(&self) -> &glfw::Window {
        return &self._glfw_window;
    }

    /// Get the GLFW window descriptor.
    pub fn get_descriptor(&self) -> &WindowDescriptor {
        return &self._descriptor;
    }

    /// Set the GLFW window descriptor.
    pub fn descriptor(mut self, new_descriptor: WindowDescriptor) -> Self {
        self._descriptor = new_descriptor;
        return self;
    }

    /// Set the OpenGL version.
    pub fn version(mut self, new_version: (u32, u32)) -> Self {
        self._gl_version = new_version;
        return self;
    }

    /// Set the OpenGL profile.
    pub fn profile(mut self, new_profile: glfw::OpenGlProfileHint) -> Self {
        self._gl_profile = new_profile;
        return self;
    }

    /// Get whether the window should close or not.
    pub fn should_close(&self) -> bool {
        return self._glfw_window.should_close();
    }

    /// Set whether the window should close or not.
    pub fn set_should_close(&mut self, condition: bool) {
        self._glfw_window.set_should_close(condition);
    }

    /// Poll the events in the window.
    pub fn poll_events(&mut self) {
        self._glfw_instance.poll_events();
    }

    /// Get the polled events.
    pub fn get_events(&self) -> &Receiver<(f64, glfw::WindowEvent)> {
        return &self._events;
    }

    /// Swap the window buffers.
    pub fn swap_buffers(&mut self) {
        self._glfw_window.swap_buffers();
    }

    /// Get the current window time.
    pub fn get_time(&self) -> f32 {
        return self._glfw_instance.get_time() as f32;
    }

    /// Build the window.
    pub fn build(mut self) -> Self {
        let (mut window, events, glfw_inst) =
            init_glfw(&self._descriptor, self._gl_version, self._gl_profile);
        init_gl(&mut window);
        self._glfw_window = window;
        self._events = events;
        self._glfw_instance = glfw_inst;
        return self;
    }
}

/// Initialize OpenGL.
pub fn init_gl(window: &mut glfw::Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
}
