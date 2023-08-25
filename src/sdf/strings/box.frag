float sdf_box (vec3 p, vec3 r)
{
    return length(max(abs(p) - r, 0.0)) + min(vmax(abs(p) - r), 0.0);
}
