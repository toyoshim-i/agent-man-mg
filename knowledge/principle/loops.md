---
id: loops
title: Seamless loops
summary: Use when a piece must loop (VJ clips, social loops, backgrounds). Loop length, periodic time, looping noise, and the seam test.
category: principle
aliases: [loop, seamless, cycle, periodic]
---
# Seamless loops

## Rule

A loop is built from periodic time, not cut from linear time. Express every animated value as
a function of a phase `p = fract(t / T)` that is identical at p = 0 and p = 1, including its
velocity. Loop length is a whole number of bars when there is music.

## Defaults

| Parameter | Value |
|---|---|
| loop length with music | 4, 8 or 16 bars (at 124 BPM: 7.74, 15.48, 30.97 s) |
| loop length without music | 4 to 12 s for social, 15 to 30 s for ambient backgrounds |
| periodic drift | `sin(2 pi p * k)` and `cos(2 pi p * k)` with integer k |
| looping noise radius | 0.5 to 1.5 in noise units (bigger means more change per loop) |
| rotation | a whole number of turns divided by symmetry: a 6-fold shape may turn 1/6 per loop |

## How

Three ways, in order of preference:

1. **Periodic parameters.** Everything driven by `sin`/`cos` of `2 pi p * k`, with integer k.
2. **Noise on a circle.** Sample 3D noise with time moved around a circle, so the field
   returns to its start:

```glsl
float hash31(vec3 p) {
    p = fract(p * 0.1031);
    p += dot(p, p.zyx + 31.32);
    return fract((p.x + p.y) * p.z);
}
float vnoise3(vec3 p) {
    vec3 i = floor(p);
    vec3 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(hash31(i), hash31(i + vec3(1, 0, 0)), f.x),
                   mix(hash31(i + vec3(0, 1, 0)), hash31(i + vec3(1, 1, 0)), f.x), f.y),
               mix(mix(hash31(i + vec3(0, 0, 1)), hash31(i + vec3(1, 0, 1)), f.x),
                   mix(hash31(i + vec3(0, 1, 1)), hash31(i + vec3(1, 1, 1)), f.x), f.y), f.z);
}
// Loops exactly every T seconds. The circle sits in (x, z) so the 2D field drifts and returns.
float loopNoise(vec2 uv, float t, float T, float radius) {
    float a = 6.2831853 * fract(t / T);
    vec3 q = vec3(uv, 0.0) + radius * vec3(cos(a), 0.0, sin(a));
    return vnoise3(q);
}
```

3. **Crossfade.** For any non-periodic `F(t)`: `F(t) * (1 - p) + F(t - T) * p`. It works for
   anything but lowers contrast mid-loop; divide the deviation from the mean by
   `sqrt((1 - p) * (1 - p) + p * p)` to restore it.

## Don't

- Don't use `mod(iTime, T)` on a linear motion and call it a loop: [loop-seam](../rule/loop-seam.md).
- Don't use non-integer frequencies (`sin(2 pi p * 1.5)`).
- Don't push the camera continuously in a loop unless the scene is self-similar at the end
  (a tunnel that repeats every segment).

## Check

- Render t = 0 and t = T: identical images.
- Render t = T - dt and t = dt: motion continues in the same direction at the same speed.

## Next

- [Noise](../recipe/noise.md)
- [VJ loop example](../example/example-vj-loop.md)
