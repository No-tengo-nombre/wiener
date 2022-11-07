use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

#[derive(Copy, Clone, Debug)]
pub enum WindowMode {
    Windowed,
    FullScreen(u32),
}

#[derive(Clone, Debug)]
pub struct WindowDescriptor {
    pub _width: u32,
    pub _height: u32,
    pub _title: String,
    pub _mode: WindowMode,
}

impl WindowDescriptor {
    pub fn builder() -> Self {
        return WindowDescriptor {
            _width: 640,
            _height: 480,
            _title: "Hello World!".to_string(),
            _mode: WindowMode::Windowed,
        };
    }

    pub fn width(mut self, width: u32) -> Self {
        self._width = width;
        return self;
    }

    pub fn height(mut self, height: u32) -> Self {
        self._height = height;
        return self;
    }

    pub fn dimensions(mut self, width: u32, height: u32) -> Self {
        self._width = width;
        self._height = height;
        return self;
    }

    pub fn title(mut self, title: &str) -> Self {
        self._title = title.to_string();
        return self;
    }

    pub fn mode(mut self, mode: WindowMode) -> Self {
        self._mode = mode;
        return self;
    }

    pub fn windowed(mut self) -> Self {
        self._mode = WindowMode::Windowed;
        return self;
    }

    pub fn fullscreen(mut self, monitor: u32) -> Self {
        self._mode = WindowMode::FullScreen(monitor);
        return self;
    }
}

/// Initializes a GLFW window, setting it as the current one.
pub fn init_glfw(
    width: u32,
    height: u32,
    title: &str,
    mode: WindowMode,
    version: (u32, u32),
    profile: glfw::OpenGlProfileHint,
) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
    let mut glfw_inst = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw_inst.window_hint(glfw::WindowHint::CenterCursor(true));
    glfw_inst.window_hint(glfw::WindowHint::ContextVersion(version.0, version.1));
    // glfw_inst.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw_inst.window_hint(glfw::WindowHint::OpenGlProfile(profile));

    let (mut window, events) = match mode {
        WindowMode::Windowed => glfw_inst
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Error creating GLFW window"),
        WindowMode::FullScreen(_) => glfw_inst
            .with_primary_monitor(|temp_glfw, m| {
                temp_glfw.create_window(
                    width,
                    height,
                    title,
                    m.map_or(glfw::WindowMode::Windowed, |m| {
                        glfw::WindowMode::FullScreen(m)
                    }),
                )
            })
            .expect("Error creating GLFW window"),
    };

    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Normal);
    window.set_cursor_enter_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.make_current();

    return (window, events, glfw_inst);
}
