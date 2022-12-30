use gl;
use wiener::gl::{GLManager, GLWindow, Mesh, Shader, ShaderProgram, VertexAttribute};
use wiener::utils::math;
use wiener::WindowDescriptor;
use wiener_gl::Drawable;

fn main() {
    env_logger::init();
    log::debug!("gl_models :: Making window");
    let mut window = GLWindow::builder()
        .descriptor(WindowDescriptor {
            width: 1000,
            height: 1000,
            title: "Models example".to_string(),
            ..Default::default()
        }).build();

    log::debug!("gl_models :: Enabling features");
    GLManager::enable(gl::DEPTH_TEST);

    log::debug!("gl_models :: Making ship shader");
    let ship_shader_arr = [
        Shader::from_file(
            "examples/gl_models/resources/shaders/ship.vert",
        ),
        Shader::from_file(
            "examples/gl_models/resources/shaders/ship.frag",
        )
    ];
    let ship_shader = ShaderProgram::from_array(&ship_shader_arr);
    
    let vertex_layout = [
        VertexAttribute { location: 0, size: 3, data_type: gl::FLOAT },
        VertexAttribute { location: 1, size: 3, data_type: gl::FLOAT },
        VertexAttribute { location: 2, size: 3, data_type: gl::FLOAT },
    ];

    log::debug!("gl_models :: Loading ship mesh");
    let mut ship =  Mesh::<f32, u32>::from_off(
        "examples/gl_models/resources/models/XJ5 X-wing starfighter.off",
        &ship_shader,
        (1.0, 1.0, 1.0),
    )
        .layout(&vertex_layout);

    log::debug!("gl_models :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);
    
    const ROTATION_SPEED: f32 = 0.5;

    log::debug!("gl_models :: Starting the render loop");
    let mut viewport;
    let mut window_time;
    while !window.should_close() {
        window.poll_events();

        // Set the time
        window_time = window.get_time();
        viewport = window.get_window().get_framebuffer_size();
        ship_shader.uniform_1f("u_time", window_time);

        GLManager::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        ship.model_mat = math::matmul(
            math::rotation(ROTATION_SPEED * window_time, 0.0, 0.0),
            math::scaling(0.5, 0.5, 0.5)
        );
        ship.draw();

        window.swap_buffers();
    }
}
