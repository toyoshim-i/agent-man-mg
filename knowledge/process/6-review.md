---
id: review
title: Step 6 - Review
summary: Use when a pass is built. Mechanical checks on rendered frames or on the code, each mapped to an explain symptom with a fix.
category: process
aliases: [critique, checklist, rubric, step-6, qa]
---
# Step 6: Review

## Rule

Judge the output, not your intent. Run every check below, write pass or fail for each, then
`agent-man-mg explain <symptom>` for each fail and apply its fix. Do at least two rounds.
Stop when a full round passes or when a round changes nothing you can see.

## Getting frames

Pick the path that matches what you can do. Both are complete reviews; the second is slower.

- **You can render and see images:** capture stills at t = 0, 25%, 50%, the peak, 90%, and the
  last frame. For motion checks, capture a contact sheet of 12 frames evenly spaced, or 30
  frames around the peak. Run the frame checks below on them, and still run the code checks:
  judging pictures is easy to get wrong, and the code checks catch what the eye misses.
- **You cannot render, or cannot see images:** run every code check at the bottom. Then
  evaluate the timing code by hand at the same six times and write a table of every animated
  property's value. The table is your contact sheet: read the motion checks off it.

## Still-frame checks

| Check | How | Fail means |
|---|---|---|
| Thumbnail | shrink the peak frame to 160 px wide | focus unclear: [no-focal-point](../rule/no-focal-point.md); text unreadable: [unreadable-type](../rule/unreadable-type.md) |
| Grayscale | convert to luminance only | focus not the highest contrast: [muddy-palette](../rule/muddy-palette.md) |
| Squint | blur by 2% of width | no clear big shapes: [static-composition](../rule/static-composition.md), [over-effected](../rule/over-effected.md) |
| Saturation | count pixels with OKLCH chroma above 0.2 | more than 10%: [oversaturated](../rule/oversaturated.md) |
| Gradients | zoom 4x into the darkest smooth area | steps visible: [banding](../rule/banding.md) |
| Edges | zoom 4x onto a diagonal or curved edge | stair-steps or shimmer: [aliasing](../rule/aliasing.md) |
| Poster | 5 random frames, each judged alone | any frame is a mess: [over-effected](../rule/over-effected.md) |
| Noise | look at any noise field | blobby, gray, grid-aligned: [cheap-noise](../rule/cheap-noise.md) |

## Motion checks

| Check | How | Fail means |
|---|---|---|
| Curves | plot each animated value against time | straight ramps: [linear-motion](../rule/linear-motion.md) |
| One star | count primary moves in each 0.25 s window | more than one: [everything-moves](../rule/everything-moves.md) |
| Stagger | list the start offsets of a group | identical gaps on 6 or more items: [mechanical-stagger](../rule/mechanical-stagger.md) |
| Entrances | find elements that appear | appear in one frame with no motion: [popping](../rule/popping.md) |
| Speed | how far ambient features travel per second | more than a quarter of the frame: [too-fast](../rule/too-fast.md) |
| Weight | look at stops and settles | no ease-in on stops, no settle: [floaty](../rule/floaty.md) |
| Energy | mean frame difference per second, over the piece | flat line: [monotony](../rule/monotony.md) |
| Seam | compare the first and last frame of a loop | any jump: [loop-seam](../rule/loop-seam.md) |
| Flashes | count mean-luminance swings above 10% per second | more than 3: [strobe-risk](../rule/strobe-risk.md) |
| Audio | watch audio-driven values on a steady beat | flicker between beats: [jitter](../rule/jitter.md) |
| Projection | view the frame darkened and lifted as a projector would | detail lost: [washed-out](../rule/washed-out.md) |

## Code checks (always; the only checks when you cannot see images)

| Check in the code | Fail means |
|---|---|
| every `mix(a, b, x)` or position driven by time passes `x` through an easing function | [linear-motion](../rule/linear-motion.md) |
| no element's visibility comes from `step(start, t)`, `if (t > ...)` or `display` toggles | [popping](../rule/popping.md) |
| no two primary events in the timing chart overlap | [everything-moves](../rule/everything-moves.md) |
| stagger offsets are not `i * constant` for 6 or more items | [mechanical-stagger](../rule/mechanical-stagger.md) |
| ambient time multipliers (`iTime * k`) have k below 0.3 | [too-fast](../rule/too-fast.md) |
| arrivals use outExpo or outQuint; springs have damping 0.6 or more | [floaty](../rule/floaty.md) |
| the energy numbers in the chart have one maximum with a lower number before it | [monotony](../rule/monotony.md) |
| edges use a pixel-width ramp (`px` or `fwidth`), never `step` or `< 0.0` | [aliasing](../rule/aliasing.md) |
| output goes through linear-to-sRGB (unless the target is an sRGB framebuffer) and a dither line last | [banding](../rule/banding.md), [washed-out](../rule/washed-out.md) |
| every color comes from the design sheet; at most 3 hues; mixes use OKLab | [muddy-palette](../rule/muddy-palette.md) |
| only the accent has OKLCH chroma above 0.12, on one element | [oversaturated](../rule/oversaturated.md) |
| the star's color differs from what is behind it by 0.4 L or more | [no-focal-point](../rule/no-focal-point.md) |
| text size at least 1/25 of frame height, weight 400 or more, hold meets read time | [unreadable-type](../rule/unreadable-type.md) |
| noise is fbm with 4 or more octaves, rotated, and shaped before color | [cheap-noise](../rule/cheap-noise.md) |
| effects in the code are all listed in the design sheet | [over-effected](../rule/over-effected.md) |
| loops use `fract(t / T)` and only integer frequencies of it | [loop-seam](../rule/loop-seam.md) |
| audio values pass auto-gain, an envelope and a curve before use | [jitter](../rule/jitter.md) |
| large-area brightness changes go through a 3-per-second limiter | [strobe-risk](../rule/strobe-risk.md) |

## Idea check (last, every round)

Read the transposition table from [Concept](2-concept.md). Does the finished piece still stand
on its "became scarce" line? Craft decisions erode ideas one reasonable step at a time: an
added pulse here, a filled pause there, until the piece is the template it was meant to answer.
If the scarce thing is gone, remove what buried it; do not add something new.

## Next

- Every fail: `agent-man-mg explain <symptom>`
- All pass: compare against the brief's `done when` in [Brief](1-brief.md)
