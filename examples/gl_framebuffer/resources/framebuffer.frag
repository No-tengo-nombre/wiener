#version 460 core

in vec2 tex_uv;
out vec4 frag_color;

layout (binding = 0) uniform sampler2D fbo_texture;

void main () {
    // frag_color = texture(fbo_texture, tex_uv);
    frag_color = vec4(tex_uv.x, tex_uv.y, 0.0f, 1.0f);
}
