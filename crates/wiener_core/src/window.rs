use glfw;
use glfw::Context;
use std::sync::mpsc::Receiver;

/// Mode for the window.
#[derive(Copy, Clone, Debug)]
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
}

impl Default for WindowDescriptor {
    fn default() -> Self {
        return WindowDescriptor {
            width: 640,
            height: 480,
            title: "Hello World!".to_string(),
            mode: WindowMode::Windowed,
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

    glfw_inst.window_hint(glfw::WindowHint::CenterCursor(true));
    glfw_inst.window_hint(glfw::WindowHint::ContextVersion(version.0, version.1));
    // glfw_inst.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw_inst.window_hint(glfw::WindowHint::OpenGlProfile(profile));

    let (mut window, events) = match descriptor.mode {
        WindowMode::Windowed => glfw_inst
            .create_window(descriptor.width, descriptor.height, &descriptor.title, glfw::WindowMode::Windowed)
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

    window.set_key_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Normal);
    window.set_cursor_enter_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.make_current();

    return (window, events, glfw_inst);
}
