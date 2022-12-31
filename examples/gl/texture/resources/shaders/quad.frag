#version 460 core

in vec4 color;
in vec2 tex_uv;

out vec4 frag_color;

layout (binding = 0) uniform sampler2D quad_texture;

void main() {
    frag_color = texture(quad_texture, tex_uv);
}
