---
id: timing
title: Timing
summary: Use when choosing how long any move, hold or pause lasts. Durations by element size, read times for text, and the frame math.
category: principle
aliases: [duration, holds, read-time, frames]
---
# Timing

## Rule

Duration follows distance and size: small things move fast, big things move slow. Hold
anything the viewer must read for its full read time. When in doubt, a move is shorter than
you think and a hold is longer.

## Defaults

| Move | Seconds | Frames at 30 / 60 |
|---|---|---|
| small element (icon, letter, dot), short distance | 0.25 to 0.40 | 8-12 / 15-24 |
| medium element (word, card), up to half the frame | 0.40 to 0.70 | 12-21 / 24-42 |
| full-frame move, wipe, camera move | 0.70 to 1.20 | 21-36 / 42-72 |
| logo resolve | 1.0 to 2.0 | 30-60 / 60-120 |
| breath before a hit | 0.07 to 0.25 | 2-8 / 4-15 |
| ambient drift period | 8 to 30 per cycle | - |

Read time for text on screen: `hold = max(1.5, words / 3 + 1.0)` seconds. A 6-word line
holds 3.0 s. A logo with a tagline holds at least 1.5 s after it fully resolves.

Mood scales all of these. Multiply durations by the speed factor in the
[Mood map](../table/mood-map.md): calm 1.5x, energetic 0.7x.

## How

1. For each event in the timing chart, look up its class above.
2. Apply the mood multiplier.
3. With music, snap to the nearest beat subdivision: 1/4, 1/2 or 1 beat. Use the
   [BPM table](../table/bpm.md). A 0.5 s move at 128 BPM becomes 0.469 s (1 beat).
4. Add the read-time hold after every text reveal.

## Don't

- Don't use the same duration for every move. Size differences need timing differences.
- Don't make entrances and exits the same length. Exits are 20 to 30% shorter.
- Don't move the viewer's reading target while it is being read.

## Check

- Plot all events on one time line: the durations differ by element class.
- Every text element's hold meets the read-time formula.
- Nothing large crosses the frame in under 0.5 s unless it is a deliberate impact.
  Too fast overall: [too-fast](../rule/too-fast.md).

## Next

- [Easing](easing.md)
- [Durations table](../table/durations.md)
