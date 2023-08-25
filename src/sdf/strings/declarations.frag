// types
struct marcher {
    vec3 normal;
    vec3 pos;
    float dist;
};
struct camera {
    vec3 pos;
    vec3 ray;
    vec3 right;
};

// srgb functions
float linear_to_srgb (float linear);
float srgb_to_linear (float srgb);


// math functions
float vmax(vec3 v);
float vmin(vec3 v);

// space operations
vec3 translate(vec3 p, vec3 translation);
vec3 rotate(vec3 p, vec3 rotation);
vec3 scale(vec3 p, vec3 scale);

// sdf primatives
float sdf_sphere(vec3 p, float r);
float sdf_box (vec3 p, vec3 r);
float sdf_cylinder(vec3 p, float height, float radius);

// sdf operations
float op_union_smooth(float d1, float d2, float k);
float op_union(float d1, float d2);
float op_diff_smooth(float d1, float d2, float k);
float op_diff(float d1, float d2);
float op_int_smooth(float d1, float d2, float k);
float op_int(float d1, float d2);

// ray marching
marcher ray_march(vec3 ro, vec3 rd);
vec3    scene_normal(vec3 p);
vec2    matcap(vec3 eye, vec3 normal);

// camera
camera rotate_z(camera cam, float deg);
camera rotate_right(camera cam, float deg);
camera cam_zoom(camera cam);
float  dep(float d, float n, float f);

// scene
float scene(vec3 p);
