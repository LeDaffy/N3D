float vmax(vec3 v) {
    return max(max(v.x, v.y), v.z);
}
float vmin(vec3 v) {
    return min(min(v.x, v.y), v.z);
}
