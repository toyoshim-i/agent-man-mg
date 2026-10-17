---
id: easing
title: Easing
summary: Use when any value changes between two states. Which curve for entrances, exits and moves, the cubic-bezier numbers, springs, and the code.
category: principle
aliases: [ease, curves, interpolation, bezier, spring]
---
# Easing

## Rule

Entrances decelerate (ease-out). Exits accelerate (ease-in). Moves from A to B ease in and
out. Pick one family for the whole piece, and use its strength to set the mood: strong
curves (expo) feel confident and designed; soft ones (sine) feel calm. Linear is only for
constant drift and spin.

## Defaults

| Use | Name | CSS cubic-bezier |
|---|---|---|
| entrance, confident (default) | outExpo | (0.16, 1, 0.3, 1) |
| entrance, softer | outCubic | (0.33, 1, 0.68, 1) |
| entrance, punchy | outQuint | (0.22, 1, 0.36, 1) |
| exit | inExpo | (0.7, 0, 0.84, 0) |
| exit, softer | inCubic | (0.32, 0, 0.67, 0) |
| move A to B | inOutCubic | (0.65, 0, 0.35, 1) |
| move, strong | inOutExpo | (0.87, 0, 0.13, 1) |
| playful overshoot | outBack | (0.34, 1.56, 0.64, 1) |
| calm, breathing | inOutSine | (0.37, 0, 0.63, 1) |

Family by first mood word: precise, premium, tech: expo. Calm, dreamy: sine. Friendly,
playful: back or spring. Aggressive: quint with short durations.

Springs, for physical feel: damping ratio 0.5 to 0.7 is bouncy, 1.0 settles with no
overshoot. Natural frequency 2 to 4 Hz for UI-sized things, 0.5 to 1.5 Hz for big masses.

## How

```glsl
float outExpo(float x)    { return x >= 1.0 ? 1.0 : 1.0 - pow(2.0, -10.0 * x); }
float inExpo(float x)     { return x <= 0.0 ? 0.0 : pow(2.0, 10.0 * x - 10.0); }
float outCubic(float x)   { float y = 1.0 - x; return 1.0 - y * y * y; }
float inOutCubic(float x) {
    return x < 0.5 ? 4.0 * x * x * x : 1.0 - pow(-2.0 * x + 2.0, 3.0) * 0.5;
}
float outBack(float x) {
    const float c1 = 1.70158;
    const float c3 = c1 + 1.0;
    float y = x - 1.0;
    return 1.0 + c3 * y * y * y + c1 * y * y;
}
float inOutSine(float x)  { return -(cos(3.14159265 * x) - 1.0) * 0.5; }
// Damped spring from 0 to 1. zeta: damping ratio (<1), w: natural frequency in rad/s.
float spring(float t, float zeta, float w) {
    float wd = w * sqrt(1.0 - zeta * zeta);
    return 1.0 - exp(-zeta * w * t) * (cos(wd * t) + zeta * w / wd * sin(wd * t));
}
```

Always clamp the input to 0..1 before calling: see [Timeline](../recipe/timeline.md).
In CSS and the Web Animations API, pass the cubic-bezier string directly.

## Don't

- Don't animate position, scale or opacity between states with raw `t`:
  [linear-motion](../rule/linear-motion.md).
- Don't overshoot on everything. Overshoot one element class (the hero), never text bodies.
- Don't end a big move with a soft ease-out: it feels like it drifts to a stop
  ([floaty](../rule/floaty.md)). Use expo or quint.

## Check

- Every time-driven interpolation passes through a named easing function.
- One family across the piece, with any exception written in the design sheet.

## Next

- [Overlap](overlap.md)
- [Arcs and spacing](arcs.md)
