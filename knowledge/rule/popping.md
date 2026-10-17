---
id: popping
title: Popping
summary: Use when elements appear or disappear in a single frame with no transition, or snap between states.
category: rule
aliases: [pop-in, snapping, hard-appear, no-transition]
---
# popping

## Symptom

An element is absent in one frame and fully present in the next. Colors or shapes snap. It
looks like a bug, not a cut.

## Cause

Visibility toggled with `if`, `step(start, t)` or `display` changes; state switches without an
interpolation; a missing entrance animation.

## Fix

1. Give every entrance and exit an animation from [Durations](../table/durations.md): at least
   0.15 s for small elements, with outExpo in and inExpo out.
2. Replace `step(start, t)` with `outExpo(seg(t, start, dur))` ([Timeline](../recipe/timeline.md)).
3. A deliberate hard cut is allowed only on a beat or a scene change, and it changes the whole
   frame, not one element.

## Next

- [Transitions](../principle/transitions.md)
