---
id: too-fast
title: Too fast
summary: Use when motion is frantic, hard to follow, or ambient elements rush across the frame.
category: rule
aliases: [frantic, rushed, hyper, speed-too-high]
---
# too-fast

## Symptom

Ambient features cross more than a quarter of the frame per second. Moves finish before the
eye arrives. A calm brief feels anxious.

## Cause

Time multipliers of 1.0 or more on ambient motion (`iTime` used directly); durations not scaled
by mood; moves shorter than the duration table.

## Fix

1. Ambient drift speed: 0.03 to 0.15 for calm, at most 0.3 for energetic.
2. Re-time moves from [Timing](../principle/timing.md) and multiply by the mood speed factor
   from the [Mood map](../table/mood-map.md).
3. Keep speed contrast: fast hits stay fast, but only in short windows; everything else slows.

## Next

- [Timing](../principle/timing.md)
- [Energy](../principle/energy.md)
