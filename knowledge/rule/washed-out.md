---
id: washed-out
title: Washed out
summary: Use when the image looks gray, foggy or flat on a projector or after export, shadows vanish, or blacks are lifted by accident.
category: rule
aliases: [faded, low-contrast, gray-blacks, double-gamma, lost-shadows]
---
# washed-out

## Symptom

Blacks are dark gray; the whole image looks foggy. Detail in dark regions disappears on the
projector. The export looks lighter than the preview.

## Cause

Double gamma (converting to sRGB into an sRGB framebuffer); tonemapping a scene that never
exceeds 1.0; lifted blacks from grading; important detail placed below L 0.2; ambient light
and haze at the venue.

## Fix

1. Check the output target: with an sRGB framebuffer, remove the manual `pow(c, 1/2.2)`
   ([Shader frame](../recipe/shader-frame.md)).
2. Remove tonemapping if nothing is brighter than 1.0.
3. Set empty space to true black for projection.
4. Move important detail to L 0.2 or above; raise contrast between main and background to
   0.6 L or more.
5. Test with the projection simulation in [Projection](../vj/vj-projection.md).

## Next

- [Projection](../vj/vj-projection.md)
- [Finish](../principle/finish.md)
