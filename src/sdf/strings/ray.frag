marcher ray_march(vec3 ro, vec3 rd)
{
    float total_distance_traveled = 0.0;
    const int NUMBER_OF_STEPS = 48;
    const float MINIMUM_HIT_DISTANCE = 0.01;
    const float MAXIMUM_TRACE_DISTANCE = 20.0;

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

vec3 scene_normal(vec3 p)
{
    const vec3 small_step = vec3(0.001, 0.0, 0.0);

    float gradient_x = scene(p + small_step.xyy) - scene(p - small_step.xyy);
    float gradient_y = scene(p + small_step.yxy) - scene(p - small_step.yxy);
    float gradient_z = scene(p + small_step.yyx) - scene(p - small_step.yyx);

    vec3 normal = vec3(gradient_x, gradient_y, gradient_z);

    return normalize(normal);
}
