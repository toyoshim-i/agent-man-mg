---
id: vj-performance
title: Real-time performance budget
summary: Use when a shader or scene must run live without dropping frames. Frame budgets, the cheapest fixes in order, resolution scaling, and what to measure.
category: vj
aliases: [performance, fps, gpu-budget, optimization, frame-drops]
---
# Real-time performance budget

## Rule

A dropped frame on a big screen is worse than a simpler image. Budget 70% of the frame time
and keep 30% headroom for the venue's machine and the other layers. Fix the most expensive
thing first: usually per-pixel loop counts, then resolution.

## Budgets

| Output | Frame time | Your budget (70%) |
|---|---|---|
| 60 fps | 16.7 ms | 11.7 ms for everything |
| 30 fps | 33.3 ms | 23.3 ms |
| one layer of three at 60 fps | - | about 4 ms |

Cost scales with pixels x per-pixel work. 4K is 4x the pixels of 1080p.

## Cheapest fixes, in order

1. Lower loop counts: raymarch steps 128 to 64, fbm octaves 6 to 4, shadow steps 48 to 24.
   Break out of loops early.
2. Render the heavy layer at 0.5 to 0.75 resolution and upscale with linear filtering. Soft
   fields (noise, fog, feedback) hide it completely; keep text and lines at full resolution.
3. Move per-frame constants (palette conversion, camera matrix, audio curves) out of the pixel
   shader into uniforms.
4. Replace texture-heavy multi-tap blurs with a downsampled blur chain.
5. Cache static layers (paper texture, background gradients) into a texture once.

## How

- Measure on the target machine at the target resolution, with all layers on.
- Watch the worst frame over a minute, not the average; spikes are what the audience sees.
- Simulation steps (particles, feedback) use a fixed time step and a cap on steps per frame,
  so a slow frame does not cascade.

## Don't

- Don't add effects to fill headroom you measured on your laptop.
- Don't use dynamic loop counts driven by audio; frame time then follows the music.

## Next

- [Live controls](vj-controls.md)
- [Raymarch](../recipe/raymarch.md)
