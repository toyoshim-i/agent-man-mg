---
id: arcs
title: Arcs and spacing
summary: Use when something travels across the frame. Paths should curve, spacing should follow the ease, and direction should continue between shots.
category: principle
aliases: [arcs-and-spacing, path, motion-path, trajectory]
---
# Arcs and spacing

## Rule

Things that travel move on arcs, not straight lines, unless the look is deliberately
mechanical (Swiss, UI, tech). Spacing between frames is wide in the middle of a move and
tight near its ends; that is what easing means on paper. Direction of travel continues
across cuts and transitions.

## Defaults

| Parameter | Value |
|---|---|
| arc bulge | 10 to 25% of the travel distance, perpendicular to it |
| arc side | toward gravity (down) for falling or tossed things; away from the frame center otherwise |
| rotation along path | align to tangent only for arrows, vehicles, creatures |
| mechanical looks | straight paths, axis-aligned, 90-degree turns |

## How

A quadratic Bezier arc from A to B with a perpendicular bulge:

```glsl
vec2 arcPath(vec2 a, vec2 b, float bulge, float x) {
    vec2 mid = 0.5 * (a + b);
    vec2 d = b - a;
    vec2 n = normalize(vec2(-d.y, d.x));
    vec2 c = mid + n * bulge * length(d);
    float y = 1.0 - x;
    return y * y * a + 2.0 * y * x * c + x * x * b;
}
```

Call it with an eased `x`: `arcPath(A, B, 0.18, inOutCubic(p))`. Spacing then tightens at
both ends automatically.

Separate axes when you want weight: ease X with inOutCubic and Y with outCubic, so the object
lifts quickly and lands slowly.

## Don't

- Don't move organic things (blobs, particles, creatures, ink) on straight lines.
- Don't reverse direction at a cut: if an element exits right, the next enters from the left.
- Don't put a sharp corner in a path without a stop: change direction only after a hold of
  2 or more frames.

## Check

- Trace the path of the primary element over the move: it is an arc, or the look is
  declared mechanical.
- Positions sampled at equal time steps bunch at both ends.

## Next

- [Overlap](overlap.md)
- [Transitions](transitions.md)
