vec3 translate(vec3 p, vec3 translation) {
    return p - translation;
}
vec3 rotate(vec3 p, vec3 rotation) {
    float cx = cos(radians(rotation.x));
    float sx = sin(radians(rotation.x));
    float cy = cos(radians(rotation.y));
    float sy = sin(radians(rotation.y));
    float cz = cos(radians(rotation.z));
    float sz = sin(radians(rotation.z));
    return mat3(1.0, 0.0, 0.0,
                0.0, cx , -sx,
                0.0, sx ,  cx) *
           mat3(cy , 0.0, sy ,
                0.0, 1.0, 0.0,
                -sy, 0.0, cy ) *   
           mat3(cz , -sz, 0.0,
                sz ,  cz, 0.0,
                0.0, 0.0, 1.0) * p;
}
vec3 scale(vec3 p, vec3 scale) {
    return mat3(scale.x, 0.0    , 0.0    ,
                0.0    , scale.y, 0.0    ,
                0.0    , 0.0    , scale.z) * p;

}
