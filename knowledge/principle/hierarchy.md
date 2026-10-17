---
id: hierarchy
title: Hierarchy and focus
summary: Use when deciding what the viewer looks at first. The order of attention, one star per moment, and how to demote everything else.
category: principle
aliases: [focus, focal-point, attention, emphasis]
---
# Hierarchy and focus

## Rule

At every moment, exactly one thing is the star. Attention goes, in this order, to:
**motion, then value contrast, then saturation, then size, then position.** Give the star
the strongest of these and take them away from everything else.

## Defaults

| Channel | Star | Everything else |
|---|---|---|
| motion | the only primary move in its 0.25 s window | still, or drifting below 30% of its speed |
| value contrast | OKLCH L differs from its surroundings by 0.4 or more | within 0.15 of the background |
| saturation | the accent color, chroma 0.12 to 0.22 | chroma below 0.06 |
| size | 1.5 to 3 times the next largest element | - |
| position | a thirds intersection or the optical center (slightly above middle) | edges, margins |
| sharpness | in focus | blurred, lower opacity (0.3 to 0.6) |

## How

1. Name the star of each section in the timing chart.
2. For each section, list the channels the star owns. It needs at least two, and motion or
   value contrast must be one of them.
3. Demote the rest: lower their contrast toward the background, reduce chroma, slow them.
   Opacity 0.3 to 0.6 and a slight blur also demote.
4. Hand off focus by motion: the old star settles, then the new one moves. Never both moving
   as primaries in the same window.

## Don't

- Don't make the background the most interesting thing. Busy noise fields steal focus:
  keep their value contrast low.
- Don't use the accent color on more than one element at a time.
- Don't let ambient motion run faster than the star.

## Check

- Grayscale the frame: the star has the highest local contrast.
- Shrink to 160 px wide: you can still point at the star in under a second.
- Fails: [no-focal-point](../rule/no-focal-point.md), [everything-moves](../rule/everything-moves.md).

## Next

- [Composition](composition.md)
- [Color](color.md)
