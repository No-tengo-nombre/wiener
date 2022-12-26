use gl;
use log;
use wiener::gl::{Drawable, GLManager, GLWindow, Mesh, Shader, ShaderProgram, VertexAttribute};
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    log::debug!("gl_triangle :: Making window");
    let mut window = GLWindow::builder()
        .descriptor(WindowDescriptor {
            width: 1000,
            height: 1000,
            title: "Triangle example".to_string(),
            ..Default::default()
        })
        .build();

    log::debug!("gl_triangle :: Making triangle shader");
    let triangle_shader_arr = [
        Shader::from_file(
            "examples/gl_uniforms/resources/triangle.vert",
        ),
        Shader::from_file(
            "examples/gl_uniforms/resources/triangle.frag",
        )
    ];
    let triangle_shader = ShaderProgram::from_array(&triangle_shader_arr);

    log::debug!("gl_triangle :: Making triangle mesh");
    let triangle = Mesh::<f32>::new(&triangle_shader)
        .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0_f32,
            ])
        .indices(&[0, 1, 2])
        .layout(&[
            VertexAttribute { location: 0, size: 3, data_type: gl::FLOAT },
            VertexAttribute { location: 1, size: 3, data_type: gl::FLOAT },
        ]);

    log::debug!("gl_triangle :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    log::debug!("gl_triangle :: Starting the render loop");
    let mut viewport;
    while !window.should_close() {
        window.poll_events();
        viewport = window.get_window().get_framebuffer_size();
        GLManager::clear(gl::COLOR_BUFFER_BIT);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        triangle.draw();

        window.swap_buffers();
    }
}
