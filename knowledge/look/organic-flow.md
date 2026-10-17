---
id: organic-flow
title: Organic flow
summary: Use when the brief is natural, calm, wellness, ambient, generative or deep house. Domain-warped fields, ink and smoke, soft light, slow breathing motion.
category: look
aliases: [generative, fluid, ink, smoke, ambient, natural]
---
# Organic flow

Living matter: smoke, ink in water, mycelium, tides. Driven by warped noise and curl flow,
colored with a narrow gradient map, moving slowly enough to watch for minutes.

## Design sheet

```text
look:      organic-flow
palette:   deep-sea, forest-mist or ember (as a gradient map, accent only at the top 5%)
type:      Fraunces 600 or Instrument Serif if any; small and quiet
shapes:    no hard shapes; fields, filaments, blobs with smooth union k 0.15
motion:    inOutSine; periodic breathing 4 to 8 s; spring 0.8 on events
speed:     factor 1.4; noise speed 0.03 to 0.08
density:   0.35 (dark areas are part of the image)
texture:   grain 0.03, vignette 0.25, bloom 0.1 on the accent
camera:    slow push 3 to 5% per phrase, or handheld drift 0.3%
```

## Signature moves

- Two-level domain warp, colored by the warp vector length for hidden structure.
- Filaments: ridged noise thresholded thin, glowing faintly.
- Breathing: the warp strength rises and falls on a 4 to 8 bar cycle.
- An accent bloom blooms in one region on the phrase change.

## Don't

- No full rainbow; three palette stops, OKLab mixing.
- No noise speed above 0.15: [too-fast](../rule/too-fast.md).
- No single-octave noise: [cheap-noise](../rule/cheap-noise.md).

## Build with

- [Noise](../recipe/noise.md), [Palette code](../recipe/palette-code.md),
  [Particles](../recipe/particles.md), [Loops](../principle/loops.md).

## Next

- [CRT and VHS](crt-vhs.md)
- [VJ loop example](../example/example-vj-loop.md)
