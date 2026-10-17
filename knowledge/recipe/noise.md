---
id: noise
title: Noise, fbm and domain warping
summary: Use when you need organic texture, flow, smoke, terrain or variation. Hashes, gradient noise, fbm with rotated octaves, domain warping, ridges, and their parameters.
category: recipe
aliases: [fbm, perlin, simplex, domain-warp, turbulence, procedural-texture]
---
# Noise, fbm and domain warping

## Rule

Raw noise looks cheap. Good noise is layered (fbm), rotated between octaves so no grid shows,
warped by itself for organic structure, and then shaped with a curve before it becomes color.
Move it slowly.

## Defaults

| Parameter | Value |
|---|---|
| octaves | 5 (4 for live at 4K, 6 to 8 for stills) |
| lacunarity | 2.0, with a rotation of about 37 degrees per octave |
| gain | 0.5 (0.45 smoother, 0.6 rougher) |
| base frequency | 1.5 to 3 cycles across the frame height |
| domain warp strength | 1.0 to 4.0 in noise units |
| time speed | 0.03 to 0.15 for ambient; never above 0.3 |
| shaping | `smoothstep(0.3, 0.7, n)` or `pow(n, 1.5)` before mapping to color |

## How

```glsl
vec2 hash22(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xx + p3.yz) * p3.zy);
}
// Gradient noise, range about -1..1.
float gnoise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    float a = dot(hash22(i) * 2.0 - 1.0, f);
    float b = dot(hash22(i + vec2(1.0, 0.0)) * 2.0 - 1.0, f - vec2(1.0, 0.0));
    float c = dot(hash22(i + vec2(0.0, 1.0)) * 2.0 - 1.0, f - vec2(0.0, 1.0));
    float d = dot(hash22(i + vec2(1.0, 1.0)) * 2.0 - 1.0, f - vec2(1.0, 1.0));
    return 1.4 * mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}
const mat2 OCTAVE = mat2(1.6, 1.2, -1.2, 1.6);   // rotate ~37 degrees, scale 2.0
float fbm(vec2 p) {
    float sum = 0.0;
    float amp = 0.5;
    for (int i = 0; i < 5; i++) {
        sum += amp * gnoise(p);
        p = OCTAVE * p;
        amp *= 0.5;
    }
    return sum;                                  // about -1..1
}
// Ridged: sharp creases, for lightning, veins, mountain ridges.
float ridged(vec2 p) { return 1.0 - abs(fbm(p)); }

// Domain warp (two levels). Returns the field and the warp vector for coloring.
float warped(vec2 p, float t, out vec2 q) {
    q = vec2(fbm(p + vec2(0.0, 0.0) + 0.05 * t), fbm(p + vec2(5.2, 1.3) - 0.04 * t));
    vec2 r = vec2(fbm(p + 1.5 * q + vec2(1.7, 9.2)), fbm(p + 1.5 * q + vec2(8.3, 2.8)));
    return fbm(p + 1.5 * r);
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    vec2 q;
    float n = warped(uv * 1.1, iTime, q);
    float v = smoothstep(0.0, 0.7, n);                                 // shape: mostly dark
    vec3 col = mix(vec3(0.004, 0.012, 0.02), vec3(0.25, 0.55, 0.62), v);
    col = mix(col, vec3(0.9, 0.55, 0.15), 0.6 * smoothstep(0.4, 0.9, length(q)));
    fragColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
```

For loops, use the circle method in [Loops](../principle/loops.md) instead of `+ 0.05 * t`.

## Don't

- Don't use one octave of value noise as a final texture: [cheap-noise](../rule/cheap-noise.md).
- Don't map noise linearly to a full rainbow palette.
- Don't animate by adding large `t` to the input; the pattern slides like a scrolling texture.
  Move octaves at different speeds, or warp with time.

## Check

- Rotate the frame 90 degrees mentally: no horizontal or vertical streaks.
- Mean brightness of the field is in the planned value key, not mid-gray.

## Next

- [Palette code](palette-code.md)
- [Organic flow](../look/organic-flow.md)
