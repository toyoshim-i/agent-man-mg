---
id: neon-grid
title: Neon grid
summary: Use when the brief is synthwave, retro-future, cyber, gaming or euphoric club. Glowing lines on dark, perspective grids, sun discs, magenta and cyan, with tonemapped light.
category: look
aliases: [synthwave, outrun, cyberpunk, retrowave, neon]
---
# Neon grid

Light as material. Thin emissive lines on near-black, hot cores that go white, colored halos.
It only looks expensive when glow is physical: added in linear space and tonemapped.

## Design sheet

```text
look:      neon-grid
palette:   synth-dusk (emission colors: accent #FF3D8B and a cyan #3DE8FF as second)
type:      Unbounded 800 or Archivo 900 wide, chrome or outlined
shapes:    lines, perspective grid, discs with horizontal slits, triangles
motion:    outExpo; grid scrolls at constant speed (the one linear motion)
speed:     factor 0.9; grid one cell per beat
density:   0.3 (darkness is half the look)
texture:   bloom 0.2, grain 0.03, vignette 0.3, CA 1 px at corners
camera:    low, slow push forward along the grid; tilt 0
```

## Signature moves

- Perspective floor grid scrolling toward the viewer, one cell per beat.
- Sun disc with horizontal slits that widen toward the bottom, gradient magenta to orange.
- Lines drawn with a core (1 px, near white) and a halo (`exp(-d * k)`, accent color).
- Horizon glow: a thin bright band where floor meets sky.

## Don't

- No glow drawn as opaque colored blur on top; add light, then tonemap.
- No more than two emission colors.
- No glowing everything: keep 70% of the frame dark.

## Build with

- [SDF 2D](../recipe/sdf-2d.md) glow, [Post chain](../recipe/post-chain.md) tonemap.
- [Rhythm](../principle/rhythm.md) for beat-locked scrolling.

## Next

- [Op-art](op-art.md)
