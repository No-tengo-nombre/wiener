use crate::GLManager;
use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;
use wiener_core::{init_glfw, WindowDescriptor};

pub struct GLWindow {
    _descriptor: WindowDescriptor,
    _glfw_window: glfw::Window,
    _events: Receiver<(f64, glfw::WindowEvent)>,
    _glfw_instance: glfw::Glfw,
    _gl_version: (u32, u32),
    _gl_profile: glfw::OpenGlProfileHint,
    _gl_manager: GLManager,
}

impl GLWindow {
    pub fn builder() -> Self {
        let mut empty_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (window, receiver) = empty_instance
            .create_window(1, 1, "", glfw::WindowMode::Windowed)
            .expect("");
        return GLWindow {
            _descriptor: WindowDescriptor::builder(),
            _glfw_window: window,
            _events: receiver,
            _glfw_instance: empty_instance,
            _gl_version: (4, 6),
            _gl_profile: glfw::OpenGlProfileHint::Core,
            _gl_manager: GLManager,
        };
    }

    pub fn get_window(&self) -> &glfw::Window {
        return &self._glfw_window;
    }

    pub fn get_descriptor(&self) -> &WindowDescriptor {
        return &self._descriptor;
    }

    pub fn descriptor(mut self, new_descriptor: WindowDescriptor) -> Self {
        self._descriptor = new_descriptor;
        return self;
    }

    pub fn version(mut self, new_version: (u32, u32)) -> Self {
        self._gl_version = new_version;
        return self;
    }

    pub fn profile(mut self, new_profile: glfw::OpenGlProfileHint) -> Self {
        self._gl_profile = new_profile;
        return self;
    }

    pub fn should_close(&self) -> bool {
        return self._glfw_window.should_close();
    }

    pub fn set_should_close(&mut self, condition: bool) {
        self._glfw_window.set_should_close(condition);
    }

    pub fn poll_events(&mut self) {
        self._glfw_instance.poll_events();
    }

    pub fn get_events(&self) -> &Receiver<(f64, glfw::WindowEvent)> {
        return &self._events;
    }

    pub fn swap_buffers(&mut self) {
        self._glfw_window.swap_buffers();
    }

    pub fn get_time(&self) -> f32 {
        return self._glfw_instance.get_time() as f32;
    }

    pub fn build(mut self) -> Self {
        let (mut window, events, glfw_inst) = init_glfw(
            self._descriptor._width,
            self._descriptor._height,
            &self._descriptor._title,
            self._descriptor._mode,
            self._gl_version,
            self._gl_profile,
        );
        init_gl(&mut window);
        self._glfw_window = window;
        self._events = events;
        self._glfw_instance = glfw_inst;
        return self;
    }
}

fn init_gl(window: &mut glfw::Window) {
    gl::load_with(|s| window.get_proc_address(s) as *const _);
}
