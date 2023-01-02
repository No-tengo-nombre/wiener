#version 460 core
layout (location = 0) in vec3 in_position;
layout (location = 1) in vec2 in_texuv;

out vec2 tex_uv;

void main() {
    gl_Position = vec4(in_position, 1.0f);
    tex_uv = in_texuv;
}
