---
id: composition
title: Composition
summary: Use when placing elements in the frame. Grids, thirds, negative space, safe areas and aspect-ratio rules, with numbers.
category: principle
aliases: [layout, framing, grid, thirds, negative-space, safe-area]
---
# Composition

## Rule

Design the frame as a few big masses, not many small things. Place the star off-center on a
thirds line unless the concept is symmetric. Leave empty space on purpose: it is what makes
the star look important. Keep text inside the safe area.

## Defaults

| Parameter | Value |
|---|---|
| masses visible when squinting | 2 to 4 |
| star position | a thirds intersection, or centered only when the whole layout is symmetric |
| optical center | 3 to 5% above the geometric center |
| negative space | 40% or more for minimal looks; 20% or more always |
| margins | 5 to 8% of frame width on every side |
| title-safe area | inner 90% of width and height (text and logos stay inside) |
| action-safe area | inner 93% |
| grid | 12 columns for 16:9; 4 to 6 columns for 9:16; gutter 1 to 2% of width |
| size steps between elements | ratio 1.5 to 2.0 (a modular scale), never 1.1 |
| lines and strokes | at least 2 px at 1080p; 0.2 to 0.5% of frame height for hero strokes |

Aspect ratios: in 9:16 the star sits in the upper half, between 30% and 45% from the top,
because the bottom 20% is covered by platform UI. In 1:1 and 4:5, center composition is
acceptable.

## How

1. Squint at the peak frame (or blur it 2% of width). Count masses. Fewer than 2 is empty,
   more than 4 is clutter.
2. Put the star on a thirds intersection. Balance it with a lighter mass on the opposite
   diagonal.
3. Snap edges to the grid columns. Alignment between unrelated elements reads as intention.
4. Size relations use one ratio: if the title is 1.0, the subtitle is 0.5 and labels are 0.25.

## Don't

- Don't spread elements evenly over the frame. Even distribution has no focus:
  [static-composition](../rule/static-composition.md).
- Don't let anything touch the frame edge by accident. Either 5% inside or clearly cropped
  (at least 20% off-frame).
- Don't center everything in a non-symmetric concept.

## Check

- Squint test shows 2 to 4 masses.
- No text outside the 90% safe box.
- Negative space is at least 20% of the frame.

## Next

- [Color](color.md)
- [Camera](camera.md)
