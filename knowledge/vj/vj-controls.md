---
id: vj-controls
title: Live controls
summary: Use when exposing parameters for live performance. Which 4 to 8 macros to expose, safe ranges so no knob position looks bad, smoothing and triggers.
category: vj
aliases: [controls, macros, knobs, midi-mapping, parameters, live-controls]
---
# Live controls

## Rule

Expose few controls, and make every position of every control look good. A performer
cannot tune 30 parameters in the dark; give 4 to 8 macros, each mapping one 0..1 value to
several internal parameters through curated ranges. Smooth continuous controls; quantize
triggers.

## Default macro set

| Macro | 0 | 1 | Drives |
|---|---|---|---|
| energy | warm-up | peak | speed, density, contrast, event rate together (see [Energy](../principle/energy.md)) |
| speed | 0.5x | 2x | time scale of ambient motion only, not beat-synced events |
| density | sparse | full | element count, cell count, layer opacity |
| color | palette A | palette B | crossfade between two design-sheet palettes in OKLab |
| feedback | none | long trails | decay 0 to 0.97 (never 1.0) |
| accent | trigger | - | one choreographed accent, quantized to the next beat |
| scene | trigger | - | next scene, quantized to the next bar |
| blackout | trigger / hold | - | fade to black in 2 beats; release fades in over 2 beats |

## How

- Map through curves: `x * x` for most (fine control at the low end), and clamp to the
  curated range. A density of 1.0 must still look intentional.
- Smooth continuous controls with a slew of 0.1 to 0.3 s so MIDI steps do not show:

```js
function slew(current, target, dt, seconds = 0.2) {
  return current + (target - current) * (1 - Math.exp(-dt / seconds));
}
```

- Triggers are quantized; see [Sync](vj-sync.md). Pressing a trigger twice before it fires
  does nothing more.
- Store one "safe state" per scene, and a panic button that returns to it.

## Don't

- Don't expose raw shader constants (octave count, epsilon, step size).
- Don't let any macro reach a state that breaks the look: all white, all black (except
  blackout), frozen, or above the [flash limits](vj-safety.md).
- Don't map one physical control to opposite meanings in different scenes.

## Next

- [Performance](vj-performance.md)
- [VJ set](vj-set.md)
