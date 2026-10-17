---
id: op-art
title: Op-art and geometric minimal
summary: Use when the brief is hypnotic, techno, minimal, scientific or optical. Black and white repetition, moire, concentric rings and line fields with slow phase motion.
category: look
aliases: [optical, geometric, black-and-white, moire, hypnotic, minimal-techno]
---
# Op-art and geometric minimal

Pure contrast and repetition: lines, rings and dots whose interference makes motion that is
not drawn. Monochrome, exact, slow. Ideal for techno rooms and long sets.

## Design sheet

```text
look:      op-art
palette:   mono-op (accent used once per phrase at most)
type:      IBM Plex Mono 500, small, or none
shapes:    lines, rings, dots; stroke 0.4 to 1% of height; all anti-aliased
motion:    inOutSine; phase shifts rather than movement; one cycle per 4 to 8 bars
speed:     factor 1.4
density:   0.5 (half black, half white)
texture:   grain 0.02, dither; no bloom
camera:    static, or rotation 1/n turn per phrase for n-fold symmetry
```

## Signature moves

- Concentric rings whose spacing is modulated by a slow wave.
- Two line fields at a small angle (2 to 8 degrees), drifting: moire emerges.
- A grid of dots whose size encodes a hidden shape that appears on the drop.
- Polar repetition with an odd count for tension, rotating 1/n per phrase.

## Don't

- No high-contrast stripes that flicker or scroll fast across most of the frame; regular
  patterns can trigger seizures: [strobe-risk](../rule/strobe-risk.md).
- No aliasing: thin lines at this contrast shimmer badly. Widen to 1.5 px AA when moving.
- No gray mush: keep values near 0 and 1, with clean transitions.

## Build with

- [Repetition](../recipe/repetition.md), [SDF 2D](../recipe/sdf-2d.md),
  [Loops](../principle/loops.md).

## Next

- [Chrome and liquid](chrome-liquid.md)
