#version 460 core

#define PI 3.1415926538

in vec3 o_pos;
in vec2 o_uv;
in vec3 o_col;

uniform vec2 u_resolution;
uniform float u_time;
uniform float u_cam_zoom;
uniform vec3  u_cam_translation;
uniform float u_fov;
uniform float u_fillet;
uniform sampler2D u_matcap;



uniform mat4 view;
uniform mat4 persp;
uniform mat3 u_cam_rot;

out vec4 o_color;
out float gl_FragDepth;

float Convert_sRGB_FromLinear (float theLinearValue) {
  return theLinearValue <= 0.0031308f
       ? theLinearValue * 12.92f
       : pow (theLinearValue, 1.0f/2.4f) * 1.055f - 0.055f;
}
float Convert_sRGB_ToLinear (float thesRGBValue) {
  return thesRGBValue <= 0.04045f
       ? thesRGBValue / 12.92f
       : pow ((thesRGBValue + 0.055f) / 1.055f, 2.4f);
}

float sdf_sphere(in vec3 p, in vec3 c, float r)
{
    return length(p - c) - r;
}
float vmax(vec3 v)
{
    return max(max(v.x, v.y), v.z);
}
float vmin(vec3 v)
{
    return min(min(v.x, v.y), v.z);
}

float sdf_box (vec3 p, vec3 r)
{
    return length(max(abs(p) - r, 0.0)) + min(vmax(abs(p) - r), 0.0);
}

float smooth_union(float d1, float d2, float k) 
{
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h);
}
float smooth_diff(float d1, float d2, float k) 
{
    float h = clamp( 0.5 - 0.5*(-d2-d1)/k, 0.0, 1.0 );
    return mix( -d2, d1, h ) + k*h*(1.0-h);
}
float smooth_int(float d1, float d2, float k) 
{
    float h = clamp( 0.5 - 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) + k*h*(1.0-h);
}
float scene(in vec3 p)
{
    float box = sdf_box(p, vec3(0.7 - 0.25)) - 0.25;
    float sphere1 = sdf_sphere(p, vec3(0.0), 0.9);
    float sphere = sdf_sphere(p+0.5, vec3(0.0), 1.0);

    //return max(box, -sphere);
    return smooth_union(smooth_int(box, sphere1, 0.05), sphere, u_fillet);
}

vec3 scene_normal(in vec3 p)
{
    const vec3 small_step = vec3(0.001, 0.0, 0.0);

    float gradient_x = scene(p + small_step.xyy) - scene(p - small_step.xyy);
    float gradient_y = scene(p + small_step.yxy) - scene(p - small_step.yxy);
    float gradient_z = scene(p + small_step.yyx) - scene(p - small_step.yyx);

    vec3 normal = vec3(gradient_x, gradient_y, gradient_z);

    return normalize(normal);
}

struct marcher {
    vec3 normal;
    vec3 pos;
    float dist;
};
marcher ray_march(in vec3 ro, in vec3 rd)
{
    float total_distance_traveled = 0.0;
    const int NUMBER_OF_STEPS = 256;
    const float MINIMUM_HIT_DISTANCE = 0.01;
    const float MAXIMUM_TRACE_DISTANCE = 100.0;

    for (int i = 0; i < NUMBER_OF_STEPS; ++i)
    {
        vec3 current_position = ro + total_distance_traveled * rd;

        float distance_to_closest = scene(current_position);

        if (distance_to_closest < MINIMUM_HIT_DISTANCE) 
        {
            vec3 normal = scene_normal(current_position);
            return marcher(normal, current_position, total_distance_traveled);
            vec3 light_position = vec3(2.0, 5.0, -3.0);
            vec3 direction_to_light = normalize(current_position - light_position);

            float diffuse_intensity = max(0.0, dot(normal, direction_to_light));

            //return vec4(vec3(1.0, 0.0, 0.0) * diffuse_intensity, total_distance_traveled);
        }

        if (total_distance_traveled > MAXIMUM_TRACE_DISTANCE)
        {
            break;
        }
        total_distance_traveled += distance_to_closest;
    }
    discard;
    return marcher(vec3(0.0), vec3(0.0), 0.0);
}


struct camera {
    vec3 pos;
    vec3 ray;
    vec3 right;
};

camera rotate_z(camera cam, float deg) {
    deg = radians(deg);
    cam.pos   = mat3(cos(deg), -sin(deg), 0.0,
                     sin(deg),  cos(deg), 0.0,
                     0.0,       0.0,      1.0) * cam.pos;
    cam.ray   = mat3(cos(deg), -sin(deg), 0.0,
                     sin(deg),  cos(deg), 0.0,
                     0.0,       0.0,      1.0) * cam.ray;
    cam.right = mat3(cos(deg), -sin(deg), 0.0,
                     sin(deg),  cos(deg), 0.0,
                     0.0,       0.0,      1.0) * cam.right;
    return cam;
}
camera rotate_right(camera cam, float deg) {
    deg = radians(deg);
    float c = cos(deg);
    float s = sin(deg);
    vec3 a = cam.right;
    cam.pos = mat3(c + (1.0-c)*a.x*a.x    ,   (1.0-c)*a.x*a.y - s*a.z, (1.0-c)*a.x*a.z + s*a.y       ,
                   (1.0-c)*a.x*a.y + s*a.z, c + (1.0-c)*a.y*a.y      ,     (1.0-c)*a.y*a.z - s*a.x   ,
                   (1.0-c)*a.x*a.z - s*a.y, (1.0-c)*a.y*a.z + s*a.x  , c + (1.0-c)*a.z*a.z) * cam.pos;
    cam.ray = mat3(c + (1.0-c)*a.x*a.x    ,   (1.0-c)*a.x*a.y - s*a.z, (1.0-c)*a.x*a.z + s*a.y       ,
                   (1.0-c)*a.x*a.y + s*a.z, c + (1.0-c)*a.y*a.y      ,     (1.0-c)*a.y*a.z - s*a.x   ,
                   (1.0-c)*a.x*a.z - s*a.y, (1.0-c)*a.y*a.z + s*a.x  , c + (1.0-c)*a.z*a.z) * cam.ray;
    cam.right = mat3(c + (1.0-c)*a.x*a.x    ,   (1.0-c)*a.x*a.y - s*a.z, (1.0-c)*a.x*a.z + s*a.y       ,
                   (1.0-c)*a.x*a.y + s*a.z, c + (1.0-c)*a.y*a.y      ,     (1.0-c)*a.y*a.z - s*a.x   ,
                   (1.0-c)*a.x*a.z - s*a.y, (1.0-c)*a.y*a.z + s*a.x  , c + (1.0-c)*a.z*a.z) * cam.right;
    return cam;

}

camera cam_zoom(camera cam) {
        cam.pos = cam.pos / length(cam.pos);
        cam.pos = u_cam_zoom * cam.pos;
        return cam;
}
vec2 matcap(vec3 eye, vec3 normal) {
  vec3 reflected = reflect(eye, normal);
  float m = 2.8284271247461903 * sqrt( reflected.z+1.0 );
  return reflected.xy / m + 0.5;
}

float dep(float d, float n, float f) {
    return (1/d - 1/n)/(1/f - 1/n);
}
void main()
{

    vec2 uv = ((o_uv - 0.5)*2.0) * u_resolution.xy / u_resolution.x;
    //uv = o_uv - 0.5;

    float aspect_ratio = u_resolution.y / u_resolution.x;
    // b = 2 itan(aspect_ratio/e)
    // tan(b/2) = aspect_ratio /e
    // e = aspect_ratio / tan(b/2)
    float e = aspect_ratio / (tan(radians(u_fov/2)));
    //e = 1 / (tan(radians(u_fov))/2);
    camera cam = camera(vec3(0.0, -5.0, 0.0), normalize(vec3(uv.x, e, uv.y)), vec3(1.0, 0.0, 0.0));
    cam.pos = u_cam_rot * cam.pos;
    cam.ray = u_cam_rot * cam.ray;
    cam.right = u_cam_rot * cam.right;
    cam = cam_zoom(cam);
    cam.pos += u_cam_translation;



    vec3 normal = ray_march(cam.pos, cam.ray).normal;
    vec3 pos = ray_march(cam.pos, cam.ray).pos;
    float dist = ray_march(cam.pos, cam.ray).dist;
    float near = 0.01;
    float far = 100.0;
    o_color = vec4((dep(dist, near, far)).xxx, 1.0);
    gl_FragDepth = dep(dist, near, far);
    //gl_FragDepth = 0.999;
    
    //gl_FragDepth = zc/wc;
    //gl_FragDepth = zc/wc;

    //tangent space normals
    float xb = dot(-cam.ray, normal) / 2.0 + 0.5;
    float xr = dot(cam.right, normal) / 2.0 + 0.5;
    float xg = dot(cross(-cam.ray, cam.right), normal) / 2.0 + 0.5;
    vec3 t_normals = vec3(xr, xg, xb);
    vec4 mat = vec4(xr, xg, xb, 1.0);
    o_color = texture(u_matcap, vec2(t_normals.x, -t_normals.y+1.0));
    o_color = vec4(Convert_sRGB_FromLinear(o_color.x), Convert_sRGB_FromLinear(o_color.y), Convert_sRGB_FromLinear(o_color.z), 1.0);
}

