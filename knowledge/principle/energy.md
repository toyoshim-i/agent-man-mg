---
id: energy
title: Energy and contrast in time
summary: Use when a piece feels flat or tiring. Plan an energy curve, use rest to make peaks land, and vary density, speed and brightness over time.
category: principle
aliases: [energy-curve, dynamics, tension, release, build, drop, contrast-in-time]
---
# Energy and contrast in time

## Rule

A peak only feels big next to something small. Plan energy as a curve with one maximum,
put a breath (stillness or darkness) right before it, and vary at least two of density,
speed and brightness between sections. Constant intensity, high or low, reads as monotony.

## Defaults

Energy is a number from 0 to 10 per section. These are the dials and their ranges:

| Dial | Low energy (2) | High energy (9) |
|---|---|---|
| speed | durations x1.5, drift 0.03 cycles/s | durations x0.6, drift 0.2 cycles/s |
| density | 10 to 20% of frame filled | 50 to 80% filled |
| brightness | mean L 0.10 to 0.25 | mean L 0.35 to 0.55 |
| contrast | soft edges, low chroma | hard edges, accent present |
| cut / event rate | one event per 2 to 4 bars | one per beat |
| camera | static or slow push | fast moves, small shake on hits |

Peak placement: 60 to 75% of the way through a piece. The breath before it: 2 to 8 frames
(video) or 1 to 2 beats (music). The release after a peak: energy drops 3 or more points.

## How

1. Write the curve as numbers per section in the timing chart. Example for a 12 s sting:
   `setup 3, build 6, breath 1, peak 9, release 4`.
2. For each section, set at least two dials from the table to match its number.
3. Build tension before the peak by reducing, not adding: thin the elements, narrow the
   palette, slow down, then hit.
4. For loops and live sets, the curve repeats per phrase. See [VJ set](../vj/vj-set.md).

## Don't

- Don't run at energy 8 or more for longer than 25% of a piece. It flattens.
- Don't approach the peak with steadily increasing intensity and no dip. The dip is the hit.
- Don't vary only one dial. Speed changes alone read as a tempo glitch.

## Check

- The energy numbers have exactly one maximum and a lower number right before it.
- Mean frame difference per second, plotted over time, follows the curve.
- Fails: [monotony](../rule/monotony.md).

## Next

- [Transitions](transitions.md)
- [Structure](../process/4-structure.md)
