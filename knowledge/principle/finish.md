---
id: finish
title: Finish (the expensive look)
summary: Use when the piece works but looks cheap, digital or flat. The finishing pass - linear color, tonemap, bloom, grain, vignette, dither - with amounts.
category: principle
aliases: [polish, post, grade, grain, vignette, bloom, look-expensive]
---
# Finish

## Rule

Most of the gap between amateur and professional frames is the last 5%: a correct color
pipeline, a little light spill, a little texture, and no 8-bit artifacts. Every piece gets a
finish pass. Each effect stays below the point where a viewer would name it.

## Defaults

| Effect | Amount | Notes |
|---|---|---|
| work in linear RGB | always | convert to sRGB only at output: `pow(c, vec3(1.0 / 2.2))` |
| tonemap | ACES-fit or Reinhard, exposure 1.0 | only if anything goes above 1.0 (glow, light) |
| bloom | threshold 0.8, intensity 0.05 to 0.2 | wide and soft beats sharp and strong |
| vignette | 0.15 to 0.35 darkening at corners | radial, smooth; 0 for flat Swiss looks |
| film grain | 0.02 to 0.05 amplitude, new per frame | luminance only; less in highlights |
| dither | 1/255 amplitude, always | removes banding in gradients |
| chromatic aberration | 0.5 to 2 px at the corners, 0 at center | only for lens or digital looks |
| lift (black level) | blacks at L 0.03 to 0.06 for filmic looks | 0 for projection |
| split tone | shadows +hue toward cool, highlights toward warm, chroma 0.02 | subtle |

## How

The order matters. Apply it as in [Post chain](../recipe/post-chain.md):

1. Render the scene in linear RGB, allowing values above 1.0 for lights.
2. Bloom (if multi-pass) or an analytic glow term added in the scene (single pass).
3. Exposure and tonemap.
4. Grade: lift, split tone, saturation.
5. Vignette.
6. Convert to sRGB.
7. Grain, then dither, last.

## Don't

- Don't use visible grain on text-heavy pieces above 0.03.
- Don't put chromatic aberration in the center of the frame or on text.
- Don't skip dither on dark gradients, vignettes and fog: [banding](../rule/banding.md).
- Don't stack bloom, glow, CA, scanlines and grain all at visible levels:
  [over-effected](../rule/over-effected.md).

## Check

- Zoom 4x into the darkest gradient: no steps.
- Turn each finish effect off one at a time: each should be missed, none should be obvious.
- Edges are smooth: [aliasing](../rule/aliasing.md).

## Next

- [Loops](loops.md)
- [Post chain](../recipe/post-chain.md)
