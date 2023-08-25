#version 460 core
out vec4 FragColor;

in vec2 o_pos;
in vec2 o_uv;
in vec4 o_col;

uniform sampler2D fonts[2];

void main()
{
    vec4 col = texture(fonts[0], o_uv);
    if (col.a < 0.01) {
        discard;
    }
    if (o_col.r < 0.01) {
        //discard;
    }
    FragColor = o_col / 255.0 * col;
}
