use gl;
use log;
use wiener::core::WindowDescriptor;
use wiener::gl::prelude::*;
use wiener::utils::math;

fn main() {
    env_logger::init();
    log::debug!("gl_texture :: Making window");
    let mut window = GLWindow::builder()
        .descriptor(WindowDescriptor {
            width: 1000,
            height: 1000,
            title: "Texture example".to_string(),
            ..Default::default()
        })
        .build();

    log::debug!("gl_texture :: Making quad shader");
    let quad_shader_arr = [
        Shader::from_file("examples/gl/texture/resources/shaders/quad.vert"),
        Shader::from_file("examples/gl/texture/resources/shaders/quad.frag"),
    ];
    let quad_shader = ShaderProgram::from_array(&quad_shader_arr);

    let quad_layout = [
        VertexAttribute {
            location: 0,
            size: 3,
            data_type: gl::FLOAT,
        },
        VertexAttribute {
            location: 1,
            size: 3,
            data_type: gl::FLOAT,
        },
        VertexAttribute {
            location: 2,
            size: 2,
            data_type: gl::FLOAT,
        },
    ];

    log::debug!("gl_texture :: Loading quad texture");
    let quad_textures = [Texture2D::default()
        .tex_num(0)
        .format(gl::RGBA)
        .build()
        .buffer_from_file("examples/gl/texture/resources/textures/chihuahua.jpg")];

    log::debug!("gl_texture :: Making quad mesh");
    let mut quad = Mesh::<f32, u32>::new(&quad_shader)
        .vertices(&[
            -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.5,
            -0.5, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0_f32,
        ])
        .indices(&[0, 2, 1, 2, 3, 1])
        .layout(&quad_layout)
        .textures(&quad_textures);

    log::debug!("gl_texture :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    const X_ROTATION_SPEED: f32 = 3.0;
    const Y_ROTATION_SPEED: f32 = 1.0;

    log::debug!("gl_texture :: Starting the render loop");
    let mut viewport;
    let mut window_time;
    while !window.should_close() {
        window.poll_events();

        // Set the time
        window_time = window.get_time();
        quad_shader.uniform_1f("u_time", window_time);

        viewport = window.get_window().get_framebuffer_size();

        GLManager::clear(gl::COLOR_BUFFER_BIT);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        quad.model_mat = math::rotation(
            (X_ROTATION_SPEED * window_time).sin(),
            Y_ROTATION_SPEED * window_time,
            0.0,
        );

        quad.draw();

        window.swap_buffers();
    }
}
