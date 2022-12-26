use gl;
use wiener::gl::{GLManager, GLWindow};
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    let window_descriptor = WindowDescriptor {
        width: 1000,
        height: 1000,
        title: "Window example".to_string(),
        ..Default::default()
    };
    let mut window = GLWindow::builder().descriptor(window_descriptor).build();

    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    while !window.should_close() {
        window.poll_events();
        GLManager::clear(gl::COLOR_BUFFER_BIT);

        window.swap_buffers();
    }
}
