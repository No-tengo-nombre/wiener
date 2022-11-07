use wiener::gl::GLWindow;
use wiener::WindowDescriptor;

fn main() {
    let window_descriptor = WindowDescriptor::builder()
        .dimensions(1000, 1000)
        .title("Window example");
    let mut window = GLWindow::builder().descriptor(window_descriptor).build();

    while !window.should_close() {
        window.poll_events();
    }
}
