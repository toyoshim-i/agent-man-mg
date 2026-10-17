---
id: loop-seam
title: Loop seam
summary: Use when a looping clip jumps, pops or hitches at the loop point, or motion reverses direction at the seam.
category: rule
aliases: [seam, loop-jump, loop-pop, not-seamless, hitch]
---
# loop-seam

## Symptom

At the loop point the image jumps, or motion changes speed or direction. Comparing the first
and last frames shows a difference.

## Cause

Time was wrapped with `mod(iTime, T)` around non-periodic motion; noise was animated by adding
time; frequencies are not integer multiples of the loop; a camera keeps pushing.

## Fix

1. Drive everything with phase `p = fract(t / T)`.
2. Use `sin`/`cos` of `2 pi p * k` with integer k for periodic motion.
3. Replace `noise(p + t)` with noise on a circle (`loopNoise` in [Loops](../principle/loops.md)).
4. For anything else, crossfade `F(t)` with `F(t - T)` by `p`.
5. Rotations: a whole number of turns per loop divided by the shape's symmetry.
6. Check t = 0 against t = T, and t = T - dt against t = dt.

## Next

- [Loops](../principle/loops.md)
- [Noise](../recipe/noise.md)
