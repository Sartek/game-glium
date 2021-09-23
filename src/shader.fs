#version 140
in vec2 my_attr;
in vec2 v_tex_coords;
out vec4 color;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_tex_coords)* vec4(my_attr, 0.0, 1.0);//vec4(my_attr, 0.0, 1.0);
}