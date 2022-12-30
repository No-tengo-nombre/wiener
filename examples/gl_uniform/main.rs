use gl;
use log;
use wiener::gl::{Drawable, GLManager, GLWindow, Mesh, Shader, ShaderProgram, VertexAttribute};
use wiener::utils::math;
use wiener::WindowDescriptor;

fn main() {
    env_logger::init();
    log::debug!("gl_uniform :: Making window");
    let mut window = GLWindow::builder()
        .descriptor(WindowDescriptor {
            width: 1000,
            height: 1000,
            title: "Uniforms example".to_string(),
            ..Default::default()
        })
        .build();

    log::debug!("gl_uniform :: Making triangle shader");
    let triangle_shader_arr = [
        Shader::from_file(
            "examples/gl_uniform/resources/triangle.vert",
        ),
        Shader::from_file(
            "examples/gl_uniform/resources/triangle.frag",
        )
    ];
    let triangle_shader = ShaderProgram::from_array(&triangle_shader_arr);

    let triangle_layout = [
        VertexAttribute { location: 0, size: 3, data_type: gl::FLOAT },
        VertexAttribute { location: 1, size: 3, data_type: gl::FLOAT },
    ];

    log::debug!("gl_uniform :: Making triangle mesh");
    let mut triangle_rotation = Mesh::<f32, u32>::new(&triangle_shader)
        .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0_f32,
            ])
        .indices(&[0, 1, 2])
        .layout(&triangle_layout);

    let mut triangle_translation = Mesh::<f32, u32>::new(&triangle_shader)
        .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
             0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
             0.0,  0.5, 0.0, 0.0, 0.0, 1.0_f32,
            ])
        .indices(&[0, 1, 2])
        .layout(&triangle_layout);

    log::debug!("gl_uniform :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    const ROTATION_SPEED: f32 = 10.0;
    const TRANSLATION_SPEED: f32 = 2.0;

    log::debug!("gl_uniform :: Starting the render loop");
    let mut viewport;
    let mut window_time;
    while !window.should_close() {
        window.poll_events();

        // Set the time
        window_time = window.get_time();
        triangle_shader.uniform_1f("u_time", window_time);

        viewport = window.get_window().get_framebuffer_size();

        GLManager::clear(gl::COLOR_BUFFER_BIT);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        triangle_rotation.model_mat = math::rotation(0.0, 0.0, (ROTATION_SPEED * window_time).sin());
        triangle_translation.model_mat = math::translation((TRANSLATION_SPEED * window_time).sin(), 0.0, 0.0);

        triangle_rotation.draw();
        triangle_translation.draw();

        window.swap_buffers();
    }
}
