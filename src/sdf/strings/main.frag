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
    o_color = vec4(linear_to_srgb(o_color.x), linear_to_srgb(o_color.y), linear_to_srgb(o_color.z), 1.0);
}

