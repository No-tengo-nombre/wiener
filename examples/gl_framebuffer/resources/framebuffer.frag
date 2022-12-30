#version 460 core

in vec2 tex_uv;
out vec4 frag_color;

uniform float u_screen_x;
uniform float u_screen_y;
layout (binding = 0) uniform sampler2D u_fbo_texture;

float dx = 1.0f / u_screen_x;
float dy = 1.0f / u_screen_y;

// Vector of offsets
vec2 OFFSETS[9] = vec2[](
    vec2(-dx,  dy),
    vec2(0.0,  dy),
    vec2( dx,  dy),
    vec2(-dx, 0.0),
    vec2(0.0, 0.0),
    vec2( dx, 0.0),
    vec2(-dx, -dy),
    vec2(0.0, -dy),
    vec2( dx, -dy)
);

// Define different kernels
const float sharpen_kernel[9] = float[](
    -1.0, -1.0, -1.0,
    -1.0,  9.0, -1.0,
    -1.0, -1.0, -1.0
);
const float blur_kernel[9] = float[](
    1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0,
    2.0 / 16.0, 4.0 / 16.0, 2.0 / 16.0,
    1.0 / 16.0, 1.0 / 16.0, 1.0 / 16.0
);

vec3 apply_kernel(sampler2D sample_texture, vec2 uv_coords, float[9] kernel) {
    vec3 sample_tex[9];
    for (int i = 0; i < 9; i++) {
        sample_tex[i] = vec3(texture(sample_texture, uv_coords + OFFSETS[i]));
    }
    vec3 color = vec3(0.0);
    for (int i = 0; i < 9; i++) {
        color += sample_tex[i] * kernel[i];
    }
    return color;
}

void main () {
    // Negative texture
    // frag_color = vec4(vec3(1.0 - texture(u_fbo_texture, tex_uv)), 1.0);

    // Grayscaled texture
    // frag_color = texture(u_fbo_texture, tex_uv);
    // frag_color = vec4(vec3((frag_color.r + frag_color.g + frag_color.b) / 3.0), 1.0);

    // Sharpen
    frag_color = vec4(apply_kernel(u_fbo_texture, tex_uv, sharpen_kernel), 1.0);

    // Blur
    // frag_color = vec4(apply_kernel(u_fbo_texture, tex_uv, blur_kernel), 1.0);

    // Unchanged texture
    // frag_color = texture(u_fbo_texture, tex_uv);
}
