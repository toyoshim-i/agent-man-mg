---
id: linear-motion
title: Linear motion
summary: Use when motion looks robotic, cheap or computer-generated, or values ramp at constant speed between states.
category: rule
aliases: [robotic, constant-speed, no-easing, mechanical-motion]
---
# linear-motion

## Symptom

Things start and stop abruptly, at full speed. Moves feel like a slideshow or a 1990s
screensaver. Plotting a value against time shows straight ramps with sharp corners.

## Cause

Interpolation uses raw progress: `mix(a, b, t / dur)`, `x += speed`, CSS `linear`, or
`iTime` directly as a position.

## Fix

1. Find every time-driven interpolation between two states.
2. Wrap its progress in an easing function: entrances `outExpo`, exits `inExpo`, moves
   `inOutCubic`. Code in [Easing](../principle/easing.md).
3. Clamp first: `outExpo(clamp((t - start) / dur, 0.0, 1.0))`, see [Timeline](../recipe/timeline.md).
4. Keep linear only for constant drift, spin and scrolling grids, and make that drift slow.

## Next

- [Easing](../principle/easing.md)
- [floaty](floaty.md) if it now drifts to a stop
