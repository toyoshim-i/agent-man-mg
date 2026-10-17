---
id: vj-layers
title: VJ layers and blending
summary: Use when compositing several live layers. Base, mid and top roles, blend modes and what each is for, opacity ranges, and the one-busy-layer rule.
category: vj
aliases: [layers, blend-modes, compositing, mixing, masks]
---
# VJ layers and blending

## Rule

Three roles, at most one busy layer. A **base** layer sets space and color and moves slowly;
a **mid** layer carries rhythm; a **top** layer holds accents and text. Only one of them may
be visually busy at a time. Blend modes are chosen by what the layer physically is: light
adds, ink multiplies.

## Roles

| Role | Content | Motion | Opacity |
|---|---|---|---|
| base | fields, gradients, slow geometry, feedback | ambient, phrase-level | 1.0 |
| mid | shapes, lines, grids on the beat | beat-level | 0.5 to 1.0 |
| top | accents, logo, text, flashes, glitch bursts | events only | 0 most of the time |

## Blend modes

| Mode | Use for | Watch out |
|---|---|---|
| add / screen | light, glow, particles, lasers over dark bases | white-out; keep the sum below 1.0 or tonemap |
| multiply | ink, shadow, paper, darkening a base with a mask | everything gets darker; lift the base |
| difference | op-art inversions, hard graphic intersections | harsh; for black and white material |
| alpha (normal) | text, logos, solid shapes | needs a contrast plate on busy bases |
| luma key | cutting a layer out by its own brightness | soft key (smoothstep 0.1 to 0.3) |
| mask | one layer revealed by another's shape | the mask's edges must be anti-aliased |

## How

- Pair a busy base with a sparse mid (a single ring, a few lines), or a calm base with a busy
  mid. Never busy with busy.
- Match palettes: all layers draw from the same design sheet palette. A layer with its own
  colors looks pasted.
- Crossfade base layers over 1 to 4 bars; cut mid layers on bars; top layer events are
  0.1 to 0.5 s.

## Don't

- Don't run three layers at full opacity; the image becomes gray noise:
  [over-effected](../rule/over-effected.md).
- Don't add two bright layers without a tonemap or a limiter.

## Next

- [Transitions](../principle/transitions.md)
- [Projection](vj-projection.md)
