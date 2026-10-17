---
id: design-sheet
title: Step 3 - Design sheet
summary: Use when the concept is chosen. Fixes palette, type, shape language, motion language and texture as numbers before any build.
category: process
aliases: [style-sheet, styleframe, step-3, design]
---
# Step 3: Design sheet

## Rule

Write the sheet below with numbers, not adjectives. Fill it from the
[Mood map](../table/mood-map.md) row of your first mood word, then adjust with the second and
third. If a [look](../look/swiss.md) fits the concept, start from its numbers instead and
change at most three values.

## Looks to start from

| Look | First mood word fits |
|---|---|
| [Swiss](../look/swiss.md) | precise, corporate, editorial |
| [Brutal type](../look/brutal-type.md) | loud, raw, aggressive |
| [Organic flow](../look/organic-flow.md) | calm, natural, ambient, deep |
| [CRT and VHS](../look/crt-vhs.md) | nostalgic, retro, lo-fi |
| [Neon grid](../look/neon-grid.md) | euphoric, retro-future, cyber |
| [Op-art](../look/op-art.md) | hypnotic, minimal, techno |
| [Chrome and liquid](../look/chrome-liquid.md) | futuristic, luxury tech, Y2K |
| [Ink and paper](../look/ink-paper.md) | handmade, literary, playful print |

None fits: write `custom` and build every row from the mood map.

## The sheet

```text
look:         name of the look page used, or "custom"
background:   hex + OKLCH (L, C, h)
main:         hex + OKLCH, role (the subject)
second:       hex + OKLCH, role (support), or none
accent:       hex + OKLCH, used on at most 10% of the frame
type:         family, weight, size as fraction of frame height, tracking
shapes:       primitives allowed (circle, line, rect, blob, glyph) and stroke width
motion:       easing family, base duration, stagger, overshoot yes/no
speed:        ambient drift cycles per second
density:      fraction of frame filled at peak (0.1 sparse ... 0.8 dense)
texture:      grain amount, vignette, bloom, CA (each a number or 0)
camera:       static | slow push | orbit | handheld, with amount
```

## How

1. Read the mood map row for mood word 1. Copy its values.
2. For mood word 2 and 3, change only the columns they are strong in. Write which ones.
3. Build the palette with [Color](../principle/color.md): pick hues, then check the value
   contrast numbers there. Or take one from [Palettes](../table/palettes.md).
4. Pick type from [Type pairings](../table/type-pairings.md).
5. Choose easing from [Easing](../principle/easing.md) and durations from
   [Durations](../table/durations.md).
6. Draw nothing yet. Read the sheet aloud against the concept sentence: does every line
   serve the verb? Delete lines that don't (set them to 0 or none).

## Don't

- Don't use more than three hues. Neutrals (near-black, near-white, grays tinted by the
  background hue) are free.
- Don't leave texture at "some grain". Write `grain 0.04`.
- Don't mix easing families (expo for some moves, back-overshoot for others) unless the
  sheet names the exception and why.

## Check

- Background and main differ by at least 0.35 in OKLCH L.
- Exactly one accent, with a written role.
- Every row has a number or an explicit `none`.

## Next

- [Structure](4-structure.md)
