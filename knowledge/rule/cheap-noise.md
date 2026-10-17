---
id: cheap-noise
title: Cheap noise
summary: Use when a noise texture looks blobby, gray, grid-aligned, like a default Perlin demo, or slides like a scrolling texture.
category: rule
aliases: [default-noise, blobby, perlin-look, boring-texture]
---
# cheap-noise

## Symptom

Soft gray blobs of one size. Faint horizontal and vertical streaks. The whole pattern
scrolls diagonally as one sheet. Mean brightness is mid-gray.

## Cause

One octave of value noise; octaves not rotated; time added to the input as a translation;
noise mapped linearly to color.

## Fix

1. Use fbm with 5 octaves, gain 0.5, and a rotation between octaves ([Noise](../recipe/noise.md)).
2. Warp the domain with itself (two levels) for organic structure.
3. Animate by moving warp layers at different slow speeds, not by sliding the input.
4. Shape the result (`smoothstep`, `pow`) before mapping it through a gradient map.
5. Set its mean brightness to the planned value key, not 0.5.

## Next

- [Noise](../recipe/noise.md)
- [Organic flow](../look/organic-flow.md)
