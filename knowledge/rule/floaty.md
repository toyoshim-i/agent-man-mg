---
id: floaty
title: Floaty
summary: Use when motion has no weight - things drift to a stop, hover, never land, or bounce without settling.
category: rule
aliases: [weightless, no-weight, mushy, drifting, endless-bounce]
---
# floaty

## Symptom

Objects slow down over a long tail and seem to never quite arrive. Big shapes move like
balloons. Springs keep wobbling. Impacts have no impact.

## Cause

Soft ease-out (sine, quad) on large moves; overly long durations; under-damped springs
(damping below 0.4); no anticipation before big moves; no hold after landing.

## Fix

1. Use outExpo or outQuint for arrivals, so most of the move happens early and it lands.
2. Springs: damping 0.6 to 1.0; settle within 0.4 s.
3. Add anticipation of 10 to 20% before big moves ([Overlap](../principle/overlap.md)).
4. After landing, hold still for at least 0.2 s before the next move.
5. For impacts, add a 2 to 4 frame squash or a shake ([Camera](../principle/camera.md)).

## Next

- [Easing](../principle/easing.md)
- [linear-motion](linear-motion.md)
