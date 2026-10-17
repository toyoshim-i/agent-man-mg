---
id: camera
title: Camera and depth
summary: Use when the frame itself moves or needs depth. Push, orbit, parallax ratios, shake on impacts, and focus, with amounts.
category: principle
aliases: [parallax, depth, shake, dolly, push, orbit, dof]
---
# Camera and depth

## Rule

A camera move must be slower and smaller than the subject's motion, or it becomes the
subject. Use depth to separate layers: things far away move less, are lower contrast and
softer. Shake only on impacts, never as a constant.

## Defaults

| Move | Amount | Time |
|---|---|---|
| slow push (dolly in) | scale 1.00 to 1.06 over the shot | whole shot, inOutSine |
| orbit | 10 to 30 degrees over the shot | whole shot, inOutSine |
| reveal pull-out | scale 1.15 to 1.0 | 1 to 2 s, outExpo |
| impact shake | amplitude 0.5 to 2% of frame height, 8 to 15 Hz | decay over 0.2 to 0.4 s |
| roll on impact | 0.5 to 2 degrees | same decay as shake |
| handheld drift | 0.2 to 0.5% of frame height, two sines at 0.3 and 0.7 Hz | constant |

Parallax speed by layer, relative to the camera move:

| Layer | Speed | Contrast | Blur |
|---|---|---|---|
| foreground | 1.3 to 1.6 | high, can be cropped by the frame | 0 to 2% of width |
| subject | 1.0 | highest | 0 |
| midground | 0.5 to 0.7 | medium | 0 to 0.5% |
| background | 0.1 to 0.3 | low (within 0.15 L of the sky or floor) | 0.5 to 1% |

Atmospheric depth: far layers move toward the background color. In a shader, mix by
`1.0 - exp(-density * distance)` with density 0.05 to 0.15 per unit.

## How

Impact shake with decay, deterministic:

```glsl
vec2 impactShake(float t, float tHit, float amp) {
    float dt = t - tHit;
    if (dt < 0.0) return vec2(0.0);
    float env = exp(-dt * 12.0);                      // gone in about 0.3 s
    float f = 11.0 * 6.2831853;                       // 11 Hz
    return amp * env * vec2(sin(f * dt), cos(f * dt * 1.3 + 1.7));
}
// uv += impactShake(t, T_HIT, 0.012);
```

## Don't

- Don't move the camera and the star at the same speed in the same direction.
- Don't shake continuously; it only reads as noise, and it is unpleasant on big screens.
- Don't zoom by more than 10% in an ambient VJ loop. Slow pushes beyond that look like errors
  on a loop seam.

## Check

- The fastest thing on screen is the star, not the camera.
- Every shake has a trigger event in the timing chart.

## Next

- [Finish](finish.md)
- [Raymarch](../recipe/raymarch.md)
