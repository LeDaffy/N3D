#version 460 core
layout (location = 0) in vec2 i_pos;
layout (location = 1) in vec2 i_uv;
layout (location = 2) in vec4 i_col;

out vec2 o_pos;
out vec2 o_uv;
out vec4 o_col;

uniform mat4 view;
uniform mat4 persp;
uniform vec2 u_screen_size;

void main()
{
    o_pos =  i_pos;
    o_uv = i_uv;
    o_col = i_col;
    gl_Position = persp * view * vec4(
                      2.0 * i_pos.x / u_screen_size.x - 1.0,
                      1.0 - 2.0 * i_pos.y / u_screen_size.y,
                      0.0,
                      1.0);
    gl_Position = vec4(
                      2.0 * i_pos.x / u_screen_size.x - 1.0,
                      1.0 - 2.0 * i_pos.y / u_screen_size.y,
                      0.0,
                      1.0);
}
