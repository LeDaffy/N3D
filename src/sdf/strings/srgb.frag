float linear_to_srgb (float linear) {
  return linear <= 0.0031308f
       ? linear * 12.92f
       : pow (linear, 1.0f/2.4f) * 1.055f - 0.055f;
}
float srgb_to_linear (float srgb) {
  return srgb <= 0.04045f
       ? srgb / 12.92f
       : pow ((srgb + 0.055f) / 1.055f, 2.4f);
}
