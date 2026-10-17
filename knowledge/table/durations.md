---
id: durations
title: Durations table
summary: Use when you need a number for how long something takes. One table of durations, offsets and holds by event type.
category: table
aliases: [duration-table, timings, frames-table]
---
# Durations table

All values in seconds at mood speed factor 1.0. Multiply by the factor in the
[Mood map](mood-map.md).

| Event | Duration | Ease |
|---|---|---|
| letter or icon in | 0.25 to 0.40 | outExpo |
| word or line in (mask rise) | 0.45 to 0.70 | outExpo |
| card, panel, shape in | 0.40 to 0.70 | outExpo or outBack (playful) |
| element out | 0.7x its entrance | inExpo |
| move across half the frame | 0.50 to 0.80 | inOutCubic |
| full-frame wipe | 0.50 to 0.90 | inOutCubic or inOutExpo |
| logo build | 1.0 to 2.0 | per part outExpo, overall staggered |
| logo hold after build | 1.5 or more | still, or drift below 1% scale |
| text hold | `max(1.5, words / 3 + 1.0)` | still |
| breath before a hit | 0.07 to 0.25 | hold or fade to dark |
| impact shake | 0.2 to 0.4 | exponential decay |
| overshoot settle | 0.2 to 0.4 | spring, damping 0.5 to 0.7 |
| stagger between items | 0.03 to 0.08 | - |
| total stagger spread | 0.6 or less | - |
| secondary lag after primary | 0.07 to 0.20 | same ease as primary |
| cross dissolve (video) | 0.3 to 0.5 | inOutSine |
| ambient drift cycle | 8 to 30 | linear or inOutSine |

Piece lengths:

| Piece | Length |
|---|---|
| logo sting | 3 to 6 |
| title card | 4 to 8 |
| lower third | in 0.5, hold 4 to 6, out 0.4 |
| social clip | 6 to 15 |
| VJ loop | 4, 8 or 16 bars, see [BPM](bpm.md) |

## Next

- [Timing](../principle/timing.md)
- [Easing](../principle/easing.md)
