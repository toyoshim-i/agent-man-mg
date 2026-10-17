---
id: ink-paper
title: Ink and paper
summary: Use when the brief is handmade, editorial, literary, Japanese, children's or craft. Paper grain, ink bleed edges, risograph misregistration, stepped frame rates.
category: look
aliases: [handmade, risograph, print, paper, sumi, hand-drawn, collage]
---
# Ink and paper

Print and hand. Light backgrounds with fiber texture, ink that bleeds at the edges, colors
that overprint and misregister, motion on twos for a drawn feel.

## Design sheet

```text
look:      ink-paper
palette:   paper or riso (multiply blending for overprints)
type:      Instrument Serif or Fraunces; for Japanese, Shippori Mincho
shapes:    blobs and strokes with noisy edges; no perfect geometry
motion:    outCubic; stepped time (12 updates per second, "on twos")
speed:     factor 1.1
density:   0.35
texture:   paper fiber noise 0.04 at 1 to 2 px scale; edge roughness from fbm; grain 0.03
camera:    static; occasional 1 px registration jitter between color layers
```

## Signature moves

- Stepped time: `t = floor(iTime * 12.0) / 12.0` for drawn motion, while paper grain stays
  static.
- Edge bleed: offset the SDF by small fbm noise (`d += 0.01 * fbm(p * 40.0)`).
- Overprint: colors combine by multiply, as ink does, and each layer is offset 1 to 2 px.
- Wipe-in like a brush: a mask driven by noise along the stroke direction.

## Don't

- No glow, bloom or additive blending; ink subtracts light.
- No pure white paper: use L 0.94 to 0.97 with a warm tint.
- No perfectly smooth vector edges for the main shapes.

## Build with

- [Noise](../recipe/noise.md) for edges and fibers, [SDF 2D](../recipe/sdf-2d.md).
- [Typography](../principle/typography.md) for literary type.

## Next

- [Swiss](swiss.md)
- [Mood map](../table/mood-map.md)
