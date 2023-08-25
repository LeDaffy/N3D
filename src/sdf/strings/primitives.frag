float sdf_sphere(vec3 p, float r) {
    return length(p) - r;
}

float sdf_box (vec3 p, vec3 r) {
    return length(max(abs(p) - r, 0.0)) + min(vmax(abs(p) - r), 0.0);
}
float sdf_cylinder(vec3 p, float height, float radius) {
  vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(radius,height);
  return min(max(d.x,d.y),0.0) + length(max(d,0.0));
}
