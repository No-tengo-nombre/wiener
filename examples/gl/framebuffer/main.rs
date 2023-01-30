use gl;
use log;
use wiener::core::WindowDescriptor;
use wiener::gl::prelude::*;
use wiener::utils::math;

const WINDOW_WIDTH: i32 = 1000;
const WINDOW_HEIGHT: i32 = 1000;

fn main() {
    env_logger::init();
    log::debug!("gl_framebuffer :: Making window");
    let mut window = GLWindow::builder()
        .descriptor(WindowDescriptor {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            title: "Framebuffer example".to_string(),
            ..Default::default()
        })
        .build();

    log::debug!("gl_framebuffer :: Enabling depth testing");
    GLManager::enable(gl::DEPTH_TEST);

    log::debug!("gl_framebuffer :: Initializing framebuffer texture");
    let fbo_texture = Texture2D::default().tex_num(0).build();
    fbo_texture.buffer_empty(WINDOW_WIDTH, WINDOW_HEIGHT);

    log::debug!("gl_framebuffer :: Initializing depth renderbuffer");
    let fbo_depth = RenderBuffer::new().set_up(gl::DEPTH24_STENCIL8, WINDOW_WIDTH, WINDOW_HEIGHT);

    log::debug!("gl_framebuffer :: Initializing framebuffer");
    let fbo = FrameBuffer::new()
        .attach_texture2d(0, &fbo_texture)
        .attach_renderbuffer(gl::DEPTH_STENCIL_ATTACHMENT, &fbo_depth);
    fbo.bind();
    fbo.verify();

    log::debug!("gl_framebuffer :: Making framebuffer shader");
    let framebuffer_shader_arr = [
        Shader::from_file("examples/gl/framebuffer/resources/shaders/framebuffer.vert"),
        Shader::from_file("examples/gl/framebuffer/resources/shaders/framebuffer.frag"),
    ];
    let framebuffer_shader = ShaderProgram::from_array(&framebuffer_shader_arr);
    framebuffer_shader.uniform_1f("u_screen_x", WINDOW_WIDTH as f32);
    framebuffer_shader.uniform_1f("u_screen_y", WINDOW_HEIGHT as f32);

    log::debug!("gl_framebuffer :: Making ship shader");
    let ship_shader_arr = [
        Shader::from_file("examples/gl/framebuffer/resources/shaders/ship.vert"),
        Shader::from_file("examples/gl/framebuffer/resources/shaders/ship.frag"),
    ];
    let ship_shader = ShaderProgram::from_array(&ship_shader_arr);

    let screen_quad_layout = [
        VertexAttribute {
            location: 0,
            size: 3,
            data_type: gl::FLOAT,
        },
        VertexAttribute {
            location: 1,
            size: 2,
            data_type: gl::FLOAT,
        },
    ];

    let vertex_layout = [
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
    ];

    log::debug!("gl_framebuffer :: Loading ship mesh");
    let mut ship = Mesh::<f32, u32>::from_file(
        "examples/gl/framebuffer/resources/models/XJ5 X-wing starfighter.obj",
        &ship_shader,
    )
    .layout(&vertex_layout);

    log::debug!("gl_framebuffer :: Making screen quad");
    let screen_quad_textures = [fbo_texture];
    let screen_quad = Mesh::<f32, u32>::new(&framebuffer_shader)
        .vertices(&[
            -1.0, -1.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0,
            1.0, 0.0, 1.0, 1.0_f32,
        ])
        .indices(&[0, 2, 1, 2, 3, 1])
        .layout(&screen_quad_layout)
        .textures(&screen_quad_textures);

    log::debug!("gl_framebuffer :: Setting clear color");
    GLManager::clear_color(0.1, 0.05, 0.05, 1.0);

    const ROTATION_SPEED: f32 = 1.0;

    log::debug!("gl_framebuffer :: Starting the render loop");
    let mut viewport;
    let mut window_time;
    while !window.should_close() {
        // === First render pass to the framebuffer === //
        fbo.bind();

        // Set the time
        window_time = window.get_time();
        viewport = window.get_window().get_framebuffer_size();
        ship_shader.uniform_1f("u_time", window_time);

        GLManager::clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        GLManager::enable(gl::DEPTH_TEST);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        ship.model_mat = math::matmul(
            math::rotation(ROTATION_SPEED * window_time, 0.0, 0.0),
            math::scaling(1.0, 1.0, 1.0),
        );
        ship.draw();

        fbo.unbind();

        // === Render framebuffer texture to a quad === //
        GLManager::clear(gl::COLOR_BUFFER_BIT);
        GLManager::disable(gl::DEPTH_TEST);
        GLManager::viewport(0, 0, viewport.0, viewport.1);

        screen_quad.draw();

        window.swap_buffers();
        window.poll_events();
    }
}
