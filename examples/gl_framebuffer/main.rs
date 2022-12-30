use gl;
use log;
use wiener::gl::prelude::*;
use wiener::utils::math;
use wiener::WindowDescriptor;

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

    log::debug!("gl_framebuffer :: Initializing framebuffer");
    let fbo = FrameBuffer::new();
    fbo.bind();

    log::debug!("gl_framebuffer :: Initializing framebuffer texture");
    let fbo_texture = Texture2D::default()
        .tex_num(0)
        .build()
        .buffer_empty(WINDOW_WIDTH, WINDOW_HEIGHT);
    fbo.inplace_attach_texture2d(0, &fbo_texture);

    log::debug!("gl_framebuffer :: Making triangle shader");
    let triangle_shader_arr = [
        Shader::from_file(
            "examples/gl_framebuffer/resources/triangle.vert",
        ),
        Shader::from_file(
            "examples/gl_framebuffer/resources/triangle.frag",
        )
    ];
    let triangle_shader = ShaderProgram::from_array(&triangle_shader_arr);

    log::debug!("gl_framebuffer :: Making framebuffer shader");
    let framebuffer_shader_arr = [
        Shader::from_file(
            "examples/gl_framebuffer/resources/framebuffer.vert",
        ),
        Shader::from_file(
            "examples/gl_framebuffer/resources/framebuffer.frag",
        ),
    ];
    let framebuffer_shader = ShaderProgram::from_array(&framebuffer_shader_arr);

    let triangle_layout = [
        VertexAttribute { location: 0, size: 3, data_type: gl::FLOAT },
        VertexAttribute { location: 1, size: 3, data_type: gl::FLOAT },
    ];
    let screen_quad_layout = [
        VertexAttribute { location: 0, size: 3, data_type: gl::FLOAT },
        VertexAttribute { location: 1, size: 2, data_type: gl::FLOAT },
    ];

    log::debug!("gl_framebuffer :: Making triangle mesh");
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

    log::debug!("gl_framebuffer :: Making screen quad");
    let screen_quad_textures = [fbo_texture];
    let screen_quad = Mesh::<f32, u32>::new(&framebuffer_shader)
        .vertices(&[
            -1.0, -1.0, 0.0, 0.0, 0.0,
            -1.0,  1.0, 0.0, 0.0, 1.0,
             1.0, -1.0, 0.0, 1.0, 0.0,
             1.0,  1.0, 0.0, 1.0, 1.0,
        ])
        .indices(&[0, 2, 1, 2, 3, 1])
        .layout(&screen_quad_layout)
        .textures(&screen_quad_textures);

    log::debug!("gl_framebuffer :: Setting clear color");
    GLManager::clear_color(0.1, 0.1, 0.3, 1.0);

    const ROTATION_SPEED: f32 = 10.0;
    const TRANSLATION_SPEED: f32 = 2.0;

    log::debug!("gl_framebuffer :: Starting the render loop");
    let mut viewport;
    let mut window_time;
    while !window.should_close() {
        // === First render pass to the framebuffer === //
        fbo.bind();
        
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
        
        fbo.unbind();

        // === Render framebuffer texture to a quad === //
        GLManager::clear(gl::COLOR_BUFFER_BIT);
        GLManager::viewport(0, 0, viewport.0, viewport.1);
        
        screen_quad.draw();

        window.swap_buffers();
        window.poll_events();
    }
}
