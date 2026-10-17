---
id: overlap
title: Overlap, follow-through and anticipation
summary: Use when motion looks stiff or robotic. Offsets between parts, settles after stops, wind-ups before big moves, and stagger for groups.
category: principle
aliases: [stagger, offset, follow-through, anticipation, secondary-motion, settle]
---
# Overlap, follow-through and anticipation

## Rule

Parts of a thing do not start or stop together. The leading part moves first; the rest
follow a few frames later and settle after it stops. Big moves are preceded by a small move
the other way. Groups enter in sequence, with uneven gaps.

## Defaults

| Technique | Amount | Timing |
|---|---|---|
| overlap: secondary part lags primary | same move, scaled 0.6 to 1.0 | 2 to 6 frames late at 30 fps (0.07 to 0.2 s) |
| follow-through: overshoot after stop | 5 to 15% of travel | settles over 0.2 to 0.4 s |
| anticipation: wind-up before a move | 10 to 20% of travel, opposite way | 0.1 to 0.25 s |
| stagger: per item in a group | 0.03 to 0.08 s between items | total spread at most 0.6 s |
| stagger order | reading order, or outward from the focus | - |
| stagger jitter | each gap scaled by 0.7 to 1.3 at random | fixed seed |
| secondary ambient motion | 10 to 30% of the primary amplitude | slower: 2 to 4x longer period |

## How

Uneven stagger with a fixed seed, so it is stable each frame:

```glsl
float hash11(float n) { return fract(sin(n * 127.1) * 43758.5453); }

// Start time of item i: base gap, jittered, accumulated.
float staggerStart(float i, float gap) {
    float s = 0.0;
    for (int k = 0; k < 64; k++) {
        if (float(k) >= i) break;
        s += gap * mix(0.7, 1.3, hash11(float(k)));
    }
    return s;
}
```

For a group of N with total spread S, set `gap = S / N`. Above 12 items, stagger by distance
from the focus instead of by index: `start = gap * distance / maxDistance * N`.

## Don't

- Don't start a group's items at identical intervals when there are 6 or more:
  [mechanical-stagger](../rule/mechanical-stagger.md).
- Don't add anticipation to small or fast things; it reads as a glitch.
- Don't let follow-through on one element overlap the next primary event.

## Check

- In the timing chart, no two parts of one object share a start time.
- The overshoot on any stop is below 15% and settles within 0.4 s.

## Next

- [Hierarchy](hierarchy.md)
- [Timeline](../recipe/timeline.md)
