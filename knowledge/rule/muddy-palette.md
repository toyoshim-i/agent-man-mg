---
id: muddy-palette
title: Muddy palette
summary: Use when colors look dirty, gray, brown or incoherent, gradients pass through gray, or there are too many hues.
category: rule
aliases: [muddy, dirty-colors, too-many-colors, rainbow, incoherent-color]
---
# muddy-palette

## Symptom

The image looks dull or dirty. Gradients between two colors go gray or brown in the middle.
There are four or more hues fighting. In grayscale, everything is mid-gray.

## Cause

Colors were mixed in sRGB; hues were invented in code instead of taken from a palette; value
structure was never designed (all colors near the same lightness); rainbow cosine palettes.

## Fix

1. Replace all color literals with the design sheet's palette, or one from
   [Palettes](../table/palettes.md).
2. Limit to three hues plus neutrals tinted with the background hue.
3. Mix and grade in OKLab: `mixOklab` in [Palette code](../recipe/palette-code.md).
4. Set value structure: background, main and accent differ by 0.25 to 0.4 L or more.
5. Color scalar fields with a gradient map, not a hue cycle.

## Next

- [Color](../principle/color.md)
- [oversaturated](oversaturated.md)
