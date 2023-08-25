float op_union_smooth(float d1, float d2, float k) {
    float h = clamp( 0.5 + 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) - k*h*(1.0-h);
}
float op_union(float d1, float d2) {
    return min(d1, d2);
}
float op_diff_smooth(float d1, float d2, float k) {
    float h = clamp( 0.5 - 0.5*(-d2-d1)/k, 0.0, 1.0 );
    return mix( -d2, d1, h ) + k*h*(1.0-h);
}
float op_diff(float d1, float d2) {
    return max(d1, -d2);
}
float op_int_smooth(float d1, float d2, float k) {
    float h = clamp( 0.5 - 0.5*(d2-d1)/k, 0.0, 1.0 );
    return mix( d2, d1, h ) + k*h*(1.0-h);
}
float op_int(float d1, float d2) {
    return max(d1, d2);
}
