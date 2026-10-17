---
id: color
title: Color
summary: Use when choosing or mixing colors. Value structure first, then hue and chroma budgets in OKLCH, area ratios, and how to blend without mud.
category: principle
aliases: [palette, colour, oklch, value, saturation, contrast]
---
# Color

## Rule

Value (lightness) carries the image; hue only decorates it. Design in three values first,
then add hue. Use OKLCH so the numbers mean what they look like. Spend chroma like money:
one accent is saturated, everything else is quiet. Mix and blend in OKLab or linear RGB,
never in sRGB.

## Defaults

| Role | OKLCH L | OKLCH C | Area |
|---|---|---|---|
| background, dark scheme | 0.12 to 0.20 | 0.00 to 0.03 | 60% |
| background, light scheme | 0.92 to 0.97 | 0.00 to 0.03 | 60% |
| main (the subject) | 0.4 or more away from background | 0.02 to 0.10; playful looks up to 0.18 | 30% |
| accent | 0.25 or more away from background | 0.12 to 0.22 | at most 10% |
| neutrals | any | below 0.03, hue of the background | free |

Hue relations:

| Mood | Hues |
|---|---|
| calm, coherent | analogous: all within 40 degrees |
| energetic, pop | accent 150 to 210 degrees from the main hue |
| warm | 20 to 80 degrees (red-orange to yellow) |
| cool, tech | 200 to 270 degrees (cyan to blue-violet) |

Never use pure primaries (`#FF0000`, `#00FF00`, `#0000FF`) or pure `#000000` / `#FFFFFF` for
large areas on screens. Tint near-black and near-white with the background hue (C about
0.01 to 0.02). On a projector, pure black for empty space is right: see
[Projection](../vj/vj-projection.md).

## How

1. Pick dark or light scheme from the mood map's value key.
2. Choose the background hue, then the main hue, then the accent hue with the table.
3. Write each as OKLCH, convert to hex with [Palette code](../recipe/palette-code.md), or
   take a tested set from [Palettes](../table/palettes.md).
4. Gradients and blends: mix in OKLab. Two complementary colors mixed in sRGB pass through
   gray; in OKLab they stay clean.
5. Light and glow: add emission in linear space, then tonemap. Bright cores go toward white
   on their own, which reads as real light.

## Don't

- Don't use a rainbow (full-hue cycling) unless the concept is about spectrum:
  [muddy-palette](../rule/muddy-palette.md).
- Don't give more than one element accent chroma at once:
  [oversaturated](../rule/oversaturated.md).
- Don't rely on hue alone to separate star from background; check grayscale.

## Check

- Grayscale the frame: three distinct value groups, star in the highest-contrast one.
- Pixels with chroma above 0.2 cover less than 10% of the frame.
- Count hues: at most three plus neutrals.

## Next

- [Typography](typography.md)
- [Palettes](../table/palettes.md)
