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
