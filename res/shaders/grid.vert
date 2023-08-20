#version 460 core
layout (location = 0) in vec3 i_pos;
layout (location = 1) in vec2 i_uv;
layout (location = 2) in vec3 i_col;

out vec3 o_pos;
out vec2 o_uv;
out vec3 o_col;

uniform mat4 view;
uniform mat4 persp;

void main()
{
    o_pos =  i_pos;
    o_uv = i_uv;
    o_col = i_col;
    gl_Position = (persp * view * vec4(i_pos, 1.0));
}
