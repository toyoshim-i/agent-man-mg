---
id: oversaturated
title: Oversaturated
summary: Use when colors burn, vibrate, look like defaults (pure red, green, blue), or the frame is tiring to look at.
category: rule
aliases: [too-saturated, garish, neon-everything, pure-colors]
---
# oversaturated

## Symptom

Large areas of strong color. Edges between saturated colors seem to vibrate. Pure `#FF0000`,
`#00FF00` or `#0000FF`. More than 10% of pixels above OKLCH chroma 0.2.

## Cause

Chroma was not budgeted. Several elements use accent-level saturation; or HSV with S = 1.0;
or additive light clipped per channel into pure primaries.

## Fix

1. Keep one accent at chroma 0.12 to 0.22, on at most 10% of the frame.
2. Pull main and background chroma below 0.1 and 0.03.
3. Replace pure primaries with palette colors.
4. For light and glow, add emission in linear space and tonemap so cores go toward white
   instead of clipping to a primary ([Post chain](../recipe/post-chain.md)).

## Next

- [Color](../principle/color.md)
- [muddy-palette](muddy-palette.md)
