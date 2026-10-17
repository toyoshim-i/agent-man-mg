---
id: sdf-2d
title: SDF 2D shapes
summary: Use when drawing shapes in a shader. Signed distance primitives, booleans, smooth blends, outlines, glow and anti-aliasing.
category: recipe
aliases: [sdf, shapes, signed-distance, primitives, glow, outline]
---
# SDF 2D shapes

## Rule

Describe every shape as a signed distance (negative inside), combine distances, and convert
to color only at the end with `fill` or `stroke` from [Shader frame](shader-frame.md). The
same distance gives you fill, outline, glow and shadow for free.

## Defaults

| Item | Value |
|---|---|
| stroke width, hero lines | 0.006 to 0.012 uv (0.3 to 0.6% of height) |
| corner radius | 0.02 to 0.08 uv; 0 for Swiss and brutal looks |
| smooth union k | 0.05 to 0.2 (organic), 0 (geometric) |
| glow | `exp(-d * k)` with k 8 to 30, intensity 0.1 to 0.5, added in linear space |
| drop shadow | same shape offset 0.01 to 0.03 down, blurred by widening `px` 10x, opacity 0.3 |

## How

```glsl
float sdCircle(vec2 p, float r) { return length(p) - r; }
float sdBox(vec2 p, vec2 b) {
    vec2 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0);
}
float sdRoundBox(vec2 p, vec2 b, float r) { return sdBox(p, b - r) - r; }
float sdSegment(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a;
    vec2 ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}
float sdRing(vec2 p, float r, float w) { return abs(length(p) - r) - w; }
// Arc of a ring from angle 0 to `amount` (0..1 of a turn), for progress rings and reveals.
float sdArc(vec2 p, float r, float w, float amount) {
    float a = atan(p.y, p.x) / 6.2831853;
    a = fract(a + 1.0);
    float onArc = step(a, amount);
    return mix(1e3, sdRing(p, r, w), onArc);
}

float opUnion(float a, float b)        { return min(a, b); }
float opSubtract(float a, float b)     { return max(a, -b); }
float opIntersect(float a, float b)    { return max(a, b); }
float opSmoothUnion(float a, float b, float k) {
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}

mat2 rot(float a) { float c = cos(a); float s = sin(a); return mat2(c, s, -s, c); }

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float px = 2.0 / iResolution.y;
    float d = opSmoothUnion(sdCircle(uv - vec2(-0.25, 0.0), 0.3),
                            sdRoundBox(rot(0.3) * (uv - vec2(0.3, 0.0)), vec2(0.25), 0.05), 0.1);
    vec3 col = vec3(0.01);
    col += vec3(1.0, 0.35, 0.15) * 0.3 * exp(-max(d, 0.0) * 18.0);   // glow, linear space
    col = mix(col, vec3(0.9), clamp(0.5 - d / px, 0.0, 1.0));         // fill
    fragColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
```

## Don't

- Don't threshold distances with `step` or `d < 0.0`: [aliasing](../rule/aliasing.md).
- Don't scale `p` without scaling the distance back: `sdCircle(p * 2.0, r) / 2.0`.
- Don't combine colors of overlapping shapes by adding; combine distances, then color.

## Check

- Outlines keep the same width when the shape scales.
- No edge is harder than 1 px at rest.

## Next

- [Repetition](repetition.md)
- [Noise](noise.md)
