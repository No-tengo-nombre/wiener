use gl;
use wiener::gl::{Drawable, GLManager, GLWindow, Mesh, Shader, ShaderProgram};
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    let window_descriptor = WindowDescriptor {
        width: 1000,
        height: 1000,
        title: "Triangle example".to_string(),
        ..Default::default()
    };
    let mut window = GLWindow::builder().descriptor(window_descriptor).build();

    let triangle_shader = ShaderProgram::new()
        .add_shader(Shader::from_file("examples/gl_triangle/resources/triangle.vert"))
        .add_shader(Shader::from_file("examples/gl_triangle/resources/triangle.frag"));

    let triangle = Mesh::new()
        .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
        ])
        .indices(&[0, 1, 2])
        .layout(&[3, 3])
        .shader(triangle_shader);

    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    while !window.should_close() {
        window.poll_events();
        GLManager::clear(gl::COLOR_BUFFER_BIT);

        triangle.draw();

        window.swap_buffers();
    }
}
