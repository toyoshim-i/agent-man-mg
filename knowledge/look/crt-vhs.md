---
id: crt-vhs
title: CRT and VHS
summary: Use when the brief is retro, nostalgic, 80s or 90s, lo-fi, analog or arcade. Scanlines, phosphor glow, chroma bleed, tracking wobble and lifted blacks.
category: look
aliases: [retro, vhs, crt, analog, lo-fi, 80s, 90s, arcade]
---
# CRT and VHS

Analog memory. The image is soft, warm, slightly misregistered, with lifted blacks and a
little instability. The craft is restraint: real tape is subtle until it fails.

## Design sheet

```text
look:      crt-vhs
palette:   ember or synth-dusk, with blacks lifted to L 0.06 to 0.1
type:      VT323 or Press Start 2P for screens; Archivo 800 for VHS titles
shapes:    pixel-aligned rectangles; text at integer pixel scales
motion:    outCubic; occasional tracking jumps (1 to 3 per 10 s)
speed:     factor 1.1
density:   0.4
texture:   scanlines 0.06 at 3 px; chroma offset 1 to 2 px; grain 0.05; vignette 0.35 with
           curved corners; bloom 0.15 (phosphor)
camera:    static with jitter 0.1% (tape wobble)
```

## Signature moves

- Horizontal chroma bleed: blur only the color channels sideways, keep luma sharp.
- Tracking error: a band 3 to 8% of the height drifts down with horizontal offset, for
  0.2 to 0.5 s, then the picture stabilizes.
- Barrel distortion 2 to 4% and rounded screen corners for CRT.
- Date or channel overlay in a monospace font, top left, inside title safe.

## Don't

- No scanlines finer than 2 px at the output resolution; they moire on screens and projectors.
- No constant heavy distortion; the subject must stay readable.
- No pure black: tape never had it.

## Build with

- [Glitch](../recipe/glitch.md) for tracking errors, gated as bursts.
- [Post chain](../recipe/post-chain.md) for bloom, grain and the curve.

## Next

- [Neon grid](neon-grid.md)
