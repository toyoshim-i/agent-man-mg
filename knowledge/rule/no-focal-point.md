---
id: no-focal-point
title: No focal point
summary: Use when a frame has no clear subject, the thumbnail is unreadable, or everything has the same weight.
category: rule
aliases: [no-focus, flat-hierarchy, unclear-subject, weak-composition]
---
# no-focal-point

## Symptom

Shrunk to 160 px wide, you cannot point at the subject in a second. In grayscale, several
regions have the same contrast. Background texture competes with the subject.

## Cause

The star does not own the attention channels: motion, value contrast, saturation, size.
Often the background is the brightest or most detailed area.

## Fix

1. Name the star. Give it value contrast 0.4 L or more against what is directly behind it.
2. Lower the background's contrast to within 0.15 L of its own mean; reduce its chroma below
   0.06.
3. Make the star 1.5 to 3 times larger than the next element, or the only one moving.
4. Put the accent color only on the star.
5. Re-run the thumbnail and grayscale checks in [Review](../process/6-review.md).

## Next

- [Hierarchy](../principle/hierarchy.md)
- [Composition](../principle/composition.md)
