---
id: post-chain
title: Post chain
summary: Use when applying the finish pass in a multi-pass shader. Lens chromatic aberration, bloom, exposure, ACES tonemap, grade, vignette, sRGB, grain and dither in the right order.
category: recipe
aliases: [post-processing, tonemap, aces, bloom, chromatic-aberration, grade]
---
# Post chain

## Rule

Post runs on a linear, high-range scene buffer and in a fixed order: lens effects, bloom,
exposure, tonemap, grade, vignette, display conversion, grain, dither. Changing the order
changes the look in ways that are hard to debug. Every amount stays subtle.

## Defaults

| Step | Value |
|---|---|
| chromatic aberration | 0.0015 uv at the corners, scaled by distance squared from center |
| bloom threshold | 0.8 (linear) |
| bloom radius | 1 to 3% of frame height |
| bloom intensity | 0.05 to 0.2 |
| exposure | 1.0 |
| tonemap | ACES fit (contrasty, filmic) or Reinhard (soft) |
| saturation after tonemap | 0.9 to 1.1 |
| vignette | 0.15 to 0.35 |
| grain | 0.02 to 0.05 |
| dither | 1/255 |

## How

Image pass, with iChannel0 = the scene buffer in linear RGB (float, linear filtering):

```glsl
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
vec3 acesFit(vec3 x) {                                   // Narkowicz approximation
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0);
}
vec3 sceneAt(vec2 st) { return texture(iChannel0, st).rgb; }

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 st = fragCoord / iResolution.xy;
    vec2 c = st - 0.5;
    float r2 = dot(c, c);

    // 1. Lens: chromatic aberration grows toward the corners.
    vec2 ca = c * r2 * 0.012;
    vec3 col = vec3(sceneAt(st + ca).r, sceneAt(st).g, sceneAt(st - ca).b);

    // 2. Bloom: 16 taps on two rings, bright parts only.
    vec3 bloom = vec3(0.0);
    float radius = 0.02;
    for (int i = 0; i < 16; i++) {
        float a = float(i) * 2.3999632;                      // golden angle
        float rr = radius * (float(i % 2) + 1.0) * 0.5;
        vec2 o = vec2(cos(a), sin(a)) * rr * vec2(iResolution.y / iResolution.x, 1.0);
        bloom += max(sceneAt(st + o) - 0.8, 0.0);
    }
    col += 0.12 * bloom / 16.0 * 4.0;

    // 3. Exposure and tonemap.
    col = acesFit(col * 1.0);

    // 4. Grade: gentle saturation and split tone.
    float luma = dot(col, vec3(0.2126, 0.7152, 0.0722));
    col = mix(vec3(luma), col, 1.05);
    col += vec3(-0.01, 0.0, 0.015) * (1.0 - luma) + vec3(0.015, 0.005, -0.01) * luma;

    // 5. Vignette.
    col *= 1.0 - 0.25 * smoothstep(0.1, 0.5, r2);

    // 6. Display, then 7. grain and 8. dither.
    col = pow(max(col, 0.0), vec3(1.0 / 2.2));
    col += (hash12(fragCoord + fract(iTime) * 613.0) - 0.5) * 0.03 * (1.0 - 0.6 * luma);
    col += (hash12(fragCoord.yx + 19.0) - 0.5) / 255.0;
    fragColor = vec4(col, 1.0);
}
```

For a real bloom, downsample the bright pass into 4 to 6 half-size buffers, blur each, and
add them back. The two-ring version above is a cheap stand-in for one pass.

In a single-pass shader with no buffer, use the analytic glow from [SDF 2D](sdf-2d.md) and
the finish lines of [Shader frame](shader-frame.md) instead.

## Don't

- Don't tonemap twice or tonemap a scene that never exceeds 1.0; it only dulls it.
- Don't apply chromatic aberration after grain; the grain smears.
- Don't let the vignette reach the star's area.

## Check

- Toggle each step: the image without it looks slightly worse, never better.
- Highlights roll off into near-white without hard clipping.

## Next

- [Finish](../principle/finish.md)
- [Projection](../vj/vj-projection.md)
