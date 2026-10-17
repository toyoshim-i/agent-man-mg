---
id: aliasing
title: Aliasing
summary: Use when edges look jagged or stair-stepped, thin lines flicker or crawl, or fine patterns shimmer and moire when moving.
category: rule
aliases: [jaggies, jagged-edges, shimmer, crawling, flicker-lines, moire]
---
# aliasing

## Symptom

Hard pixel steps on curves and diagonals. Thin lines break up and crawl as they move. Fine
repeated patterns shimmer or show moire.

## Cause

Edges use `step` or `d < 0.0`; edge width not tied to pixel size; lines thinner than about
1.5 px; pattern frequency above half the pixel rate.

## Fix

1. Convert distances to coverage with a pixel-width ramp:
   `clamp(0.5 - d / px, 0.0, 1.0)` where `px = 2.0 / iResolution.y`
   ([Shader frame](../recipe/shader-frame.md)).
2. For distorted or scaled spaces, use `fwidth(d)` as the width.
3. Make moving lines at least 1.5 px wide, static lines at least 1 px.
4. Fade out pattern detail as it approaches pixel size: multiply its contrast by
   `smoothstep(1.0 * px, 3.0 * px, cellSize)`, which is 0 below 1 px and 1 above 3 px.
5. When resolution allows, supersample: render at 2x and downsample.

## Next

- [SDF 2D](../recipe/sdf-2d.md)
- [Repetition](../recipe/repetition.md)
