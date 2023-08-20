#version 460 core
out vec4 FragColor;

in vec4 o_pos4;
in vec3 o_pos;
in vec2 o_uv;

uniform sampler2D default_tex;
in vec4 gl_FragCoord;

void main()
{
    FragColor = vec4(0.1, 0.2, 0.3, 1.0);
    FragColor = vec4(o_uv, 0.0, 1.0);
    FragColor = texture(default_tex, o_uv);
}
