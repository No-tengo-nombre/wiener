#version 460 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_normal;

out vec3 color;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
    gl_Position = u_projection * u_view * u_model * vec4(in_position, 1.0f);
    color = vec3(0.0, 0.0, 0.5 * (in_position.y + 1.0));
}
