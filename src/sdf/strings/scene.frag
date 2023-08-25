float scene(vec3 p) {
    float box = sdf_box(p, vec3(0.7 - 0.15)) - 0.15;
    float sphere1 = sdf_sphere(p, vec3(0.0), 0.9);
    float sphere = sdf_sphere(p-0.5, vec3(0.0), 1.0);

    //return max(box, -sphere);
    return smooth_union(smooth_int(box, sphere1, 0.05), sphere, u_fillet);
}
