---
id: banding
title: Banding
summary: Use when smooth gradients, vignettes, fog or glows show visible steps or contour lines, especially in dark areas.
category: rule
aliases: [posterization, gradient-steps, contour-lines, stepping]
---
# banding

## Symptom

Smooth gradients break into visible bands, most visible in dark areas, vignettes, fog, glows
and long feedback tails. Worse after video compression.

## Cause

8-bit output cannot represent the gradient's slope; no dither. Also: feedback or trails in
8-bit buffers, or a gradient computed in sRGB that compresses dark values.

## Fix

1. Add dither as the very last operation, after sRGB conversion:
   `col += (hash12(fragCoord) - 0.5) / 255.0;` ([Shader frame](../recipe/shader-frame.md)).
2. Add grain 0.02 to 0.04 for dark scenes; it hides residual steps.
3. Use float buffers for feedback and multi-pass chains.
4. Avoid very long, very dark gradients in the design; give them a little texture.

## Next

- [Finish](../principle/finish.md)
- [washed-out](washed-out.md)
