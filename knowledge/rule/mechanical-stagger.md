---
id: mechanical-stagger
title: Mechanical stagger
summary: Use when a group animates like a machine - identical gaps, identical moves, a perfect wave - and looks computer-generated.
category: rule
aliases: [uniform-stagger, even-spacing, too-regular, perfect-wave]
---
# mechanical-stagger

## Symptom

Items enter one by one at exactly equal intervals, all with the same move. A grid ripples as a
perfect sine. It reads as a loading animation.

## Cause

Start time `i * gap` with constant gap; same amplitude and duration for every item; phase tied
only to index.

## Fix

1. Jitter each gap by 0.7 to 1.3 with a fixed-seed hash (`staggerStart` in
   [Overlap](../principle/overlap.md)).
2. Vary duration or amplitude per item by 0.85 to 1.0.
3. Order by meaning: reading order, or distance from the focus, not array index.
4. For grids, combine a distance-based wave with a per-cell random phase of 10 to 20% of the
   cycle ([Repetition](../recipe/repetition.md)).
5. Cap the total spread at 0.6 s.

## Next

- [Overlap](../principle/overlap.md)
