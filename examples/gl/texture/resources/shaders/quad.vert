#version 460 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_uv;

out vec4 color;
out vec2 tex_uv;

uniform mat4 u_model;


void main() {
    gl_Position = u_model * vec4(in_position, 1.0f);
    color = vec4(in_color, 1.0f);
    tex_uv = in_uv;
}
