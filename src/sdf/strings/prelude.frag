#version 460 core

#define PI 3.1415926538

in vec3 o_pos; // vertex pos
in vec2 o_uv;  // vertex uv
in vec3 o_col; // vertex color

// Camera controls
uniform vec2      u_resolution;
uniform float     u_cam_zoom;                        
uniform mat3      u_cam_rot;
uniform vec3      u_cam_translation;     
uniform float     u_fov;                             

uniform float     u_fillet;                          
uniform sampler2D u_matcap;                          


// Outputs
out vec4 o_color;
out float gl_FragDepth;
