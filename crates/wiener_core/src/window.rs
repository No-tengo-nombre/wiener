use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

use log;

/// Mode for the window.
#[derive(Clone, Copy, Debug)]
pub enum WindowMode {
    Windowed,
    FullScreen(u32),
}

/// Descriptor of a window.
#[derive(Clone, Debug)]
pub struct WindowDescriptor {
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub mode: WindowMode,
    pub key_polling: bool,
    pub cursor_enter_polling: bool,
    pub cursor_pos_polling: bool,
    pub mouse_button_polling: bool,
    pub framebuffer_size_polling: bool,
    pub cursor_mode: glfw::CursorMode,
    pub make_current: bool,
}

impl WindowDescriptor {
    /// Generate a builder for the descriptor.
    pub fn builder() -> Self {
        return WindowDescriptor {
            ..Default::default()
        };
    }

    /// Set the width.
    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        return self;
    }

    /// Set the height.
    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        return self;
    }

    /// Set the dimensions (width and height).
    pub fn set_dimensions(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        return self;
    }

    /// Set the title of the window.
    pub fn set_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        return self;
    }

    /// Set the mode.
    pub fn set_mode(mut self, mode: WindowMode) -> Self {
        self.mode = mode;
        return self;
    }

    /// Make the window windowed.
    pub fn set_windowed(mut self) -> Self {
        self.mode = WindowMode::Windowed;
        return self;
    }

    /// Make the window fullscreen.
    pub fn set_fullscreen(mut self, monitor: u32) -> Self {
        self.mode = WindowMode::FullScreen(monitor);
        return self;
    }

    /// Set the key polling event.
    pub fn set_key_polling(mut self, key_polling: bool) -> Self {
        self.key_polling = key_polling;
        return self;
    }

    /// Set the cursor enter polling event.
    pub fn set_cursor_enter_polling(mut self, cursor_enter_polling: bool) -> Self {
        self.cursor_enter_polling = cursor_enter_polling;
        return self;
    }

    /// Set the cursor position polling event.
    pub fn set_cursor_pos_polling(mut self, cursor_pos_polling: bool) -> Self {
        self.cursor_pos_polling = cursor_pos_polling;
        return self;
    }

    /// Set the mouse button polling event.
    pub fn set_mouse_button_polling(mut self, mouse_button_polling: bool) -> Self {
        self.mouse_button_polling = mouse_button_polling;
        return self;
    }

    /// Set the framebuffer size polling event.
    pub fn set_framebuffer_size_polling(mut self, framebuffer_size_polling: bool) -> Self {
        self.framebuffer_size_polling = framebuffer_size_polling;
        return self;
    }

    /// Set the cursor mode.
    pub fn set_cursor_mode(mut self, cursor_mode: glfw::CursorMode) -> Self {
        self.cursor_mode = cursor_mode;
        return self;
    }

    /// Set whether to set the created window as current or not.
    pub fn set_make_current(mut self, make_current: bool) -> Self {
        self.make_current = make_current;
        return self;
    }
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        return WindowDescriptor {
            width: 640,
            height: 480,
            title: "Hello World!".to_string(),
            mode: WindowMode::Windowed,
            key_polling: true,
            cursor_enter_polling: true,
            cursor_pos_polling: true,
            mouse_button_polling: true,
            framebuffer_size_polling: true,
            cursor_mode: glfw::CursorMode::Normal,
            make_current: true,
        };
    }
}

/// Initializes a GLFW window, setting it as the current one.
pub fn init_glfw(
    descriptor: &WindowDescriptor,
    version: (u32, u32),
    profile: glfw::OpenGlProfileHint,
) -> (glfw::Window, Receiver<(f64, glfw::WindowEvent)>, glfw::Glfw) {
    let mut glfw_inst = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    log::info!("init_glfw :: Initializing GLFW");

    glfw_inst.window_hint(glfw::WindowHint::CenterCursor(true));
    glfw_inst.window_hint(glfw::WindowHint::ContextVersion(version.0, version.1));
    // glfw_inst.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw_inst.window_hint(glfw::WindowHint::OpenGlProfile(profile));

    let (mut window, events) = match descriptor.mode {
        WindowMode::Windowed => glfw_inst
            .create_window(
                descriptor.width,
                descriptor.height,
                &descriptor.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Error creating GLFW window"),
        WindowMode::FullScreen(_) => glfw_inst
            .with_primary_monitor(|temp_glfw, m| {
                temp_glfw.create_window(
                    descriptor.width,
                    descriptor.height,
                    &descriptor.title,
                    m.map_or(glfw::WindowMode::Windowed, |m| {
                        glfw::WindowMode::FullScreen(m)
                    }),
                )
            })
            .expect("Error creating GLFW window"),
    };

    log::info!("init_glfw :: Configuring window");
    window.set_key_polling(descriptor.key_polling);
    window.set_cursor_mode(descriptor.cursor_mode);
    window.set_cursor_enter_polling(descriptor.cursor_enter_polling);
    window.set_mouse_button_polling(descriptor.mouse_button_polling);
    window.set_cursor_pos_polling(descriptor.cursor_pos_polling);
    window.set_framebuffer_size_polling(descriptor.framebuffer_size_polling);
    if descriptor.make_current {
        window.make_current();
    }

    return (window, events, glfw_inst);
}
