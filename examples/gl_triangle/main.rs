use gl;
use log;
use wiener::gl::{Drawable, GLManager, GLWindow, Mesh, Shader, ShaderProgram};
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    log::debug!("gl_triangle :: Making window descriptor");
    let window_descriptor = WindowDescriptor {
        width: 1000,
        height: 1000,
        title: "Triangle example".to_string(),
        ..Default::default()
    };
    let mut window = GLWindow::builder().descriptor(window_descriptor).build();
    
    log::debug!("gl_triangle :: Making triangle shader");
    let triangle_shader = ShaderProgram::new()
    .add_shader(Shader::from_file("examples/gl_triangle/resources/triangle.vert"))
    .add_shader(Shader::from_file("examples/gl_triangle/resources/triangle.frag"));
    
    log::debug!("gl_triangle :: Making triangle mesh");
    let triangle = Mesh::new()
    .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
             ])
        .indices(&[0, 1, 2])
        .layout(&[3, 3])
        .shader(triangle_shader);

    log::debug!("gl_triangle :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);
    
    log::debug!("gl_triangle :: Starting the render loop");
    while !window.should_close() {
        window.poll_events();
        GLManager::clear(gl::COLOR_BUFFER_BIT);

        triangle.draw();

        window.swap_buffers();
    }
}
