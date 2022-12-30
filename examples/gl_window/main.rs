use gl;
use wiener::gl::{GLManager, GLWindow};
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    log::debug!("gl_window :: Making window");
    let mut window = GLWindow::builder().descriptor(WindowDescriptor {
        width: 1000,
        height: 1000,
        title: "Window example".to_string(),
        ..Default::default()
    }).build();

    log::debug!("gl_window :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    log::debug!("gl_window :: Starting the render loop");
    while !window.should_close() {
        window.poll_events();
        GLManager::clear(gl::COLOR_BUFFER_BIT);

        window.swap_buffers();
    }
}
