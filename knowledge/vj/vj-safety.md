---
id: vj-safety
title: Flash and pattern safety
summary: Use when any visual flashes, strobes, inverts or shows high-contrast stripes. Photosensitive limits (3 flashes per second, red flashes, patterns) and a limiter.
category: vj
aliases: [safety, epilepsy, photosensitive, strobe, flashing, seizure]
---
# Flash and pattern safety

## Rule

No more than **3 flashes in any one second** when the flashing area is large (a quarter of the
screen or more). Never flash saturated red. Avoid high-contrast stripes or checkerboards that
flicker, scroll or cover large areas. These limits come from WCAG 2.3.1 and ITU-R BT.1702, and
they hold even in clubs: a VJ screen reaches people who did not choose strobes.

## Definitions

| Term | Meaning |
|---|---|
| flash | a pair of opposing luminance changes (up then down) of 10% or more of maximum, where the darker state is below 80% |
| large area | 25% or more of the screen at the same time |
| red flash | any flash involving a transition to or from saturated red |
| hazardous pattern | 5 or more light-dark stripe pairs, high contrast, covering a large area, moving or flickering |

## How

Rate-limit every full-frame or large-area brightness event in the control code:

```js
class FlashLimiter {
  constructor(minInterval = 0.34) { this.min = minInterval; this.last = -Infinity; }
  // Returns true if a flash may fire now. Call with the current time in seconds.
  allow(t) {
    if (t - this.last < this.min) return false;
    this.last = t;
    return true;
  }
}
const flashes = new FlashLimiter();
// if (kickOnset && flashes.allow(now)) triggerFlash();
```

- Kick-driven brightness: either keep its swing below 10% of maximum luminance, or put it
  through the limiter, or restrict it to under 25% of the screen.
- Inversions, white frames and strobes: through the limiter, always.
- Glitch bursts: the frame may shift but not invert repeatedly.
- Stripe fields: keep contrast moderate (L difference below 0.5) or keep them still.

## Don't

- Don't strobe the full screen on every beat above 180 BPM, or on 1/8 notes at any tempo.
  That is more than 3 per second.
- Don't use red-black or red-white flashing.
- Don't assume the lighting designer's strobes make yours acceptable.

## Next

- [Strobe risk](../rule/strobe-risk.md)
- [Live controls](vj-controls.md)
