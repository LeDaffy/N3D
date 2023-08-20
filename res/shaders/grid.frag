#version 460 core
out vec4 FragColor;
in vec4 gl_FragCoord;

in vec3 o_pos;
in vec2 o_uv;
in vec3 o_col;


void main()
{
    vec2 grid = fract(o_uv * 100);
    vec3 col = vec3(0.6);

    grid += 0.5;
    float width = 2.5 / 1920.0 * 2.0;
    if ((grid.x > (0.5 + width) || grid.x < (0.5 - width)) && (grid.y > (0.5 + width) || grid.y < (0.5 - width))) {
        discard;
    }

    if (o_col.r > 0.01 || o_col.g > 0.01 || o_col.b > 0.01) {
        col = o_col;
    }
    FragColor = vec4(col, 1.0);
}
