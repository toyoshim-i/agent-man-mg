---
id: timeline
title: Timeline (timing chart to code)
summary: Use when turning a timing chart into shader or JS code. Segment helpers, in-hold-out envelopes, beat-locked times, and deterministic time for frame capture.
category: recipe
aliases: [keyframes, sequencing, animation-code, segments]
---
# Timeline

## Rule

One chart line becomes one line of code: `p = ease(seg(t, start, dur))`. Time comes from one
variable `t`, never from several clocks. Every value is a pure function of `t`, so any frame
can be rendered alone (needed for review and for scrubbing).

## Defaults

| Item | Value |
|---|---|
| progress | `seg(t, start, dur)` clamped to 0..1 |
| easing | named function from [Easing](../principle/easing.md) |
| envelopes | in, hold, out with `inHoldOut` |
| times | in seconds; with music, `beats * 60.0 / BPM` |

## How

```glsl
float seg(float t, float start, float dur) { return clamp((t - start) / dur, 0.0, 1.0); }
float outExpo(float x) { return x >= 1.0 ? 1.0 : 1.0 - pow(2.0, -10.0 * x); }
float inExpo(float x)  { return x <= 0.0 ? 0.0 : pow(2.0, 10.0 * x - 10.0); }
float inOutCubic(float x) {
    return x < 0.5 ? 4.0 * x * x * x : 1.0 - pow(-2.0 * x + 2.0, 3.0) * 0.5;
}
// 0 -> 1 with outExpo over [tIn, tIn + dIn], hold, then 1 -> 0 with inExpo from tOut.
float inHoldOut(float t, float tIn, float dIn, float tOut, float dOut) {
    return outExpo(seg(t, tIn, dIn)) * (1.0 - inExpo(seg(t, tOut, dOut)));
}

const float BPM = 124.0;
float beats(float n) { return n * 60.0 / BPM; }

// ---- timing chart ----
const float DURATION = 6.0;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float t = mod(iTime, DURATION);

    float ringScale = outExpo(seg(t, 0.00, 0.60));                  // ring  scale  0 -> 1
    float ringAlpha = inHoldOut(t, 0.10, 0.45, 5.20, 0.40);         // ring  opacity
    float slide     = inOutCubic(seg(t, beats(2.0), beats(1.0)));   // bar   x  -1 -> 0

    float ring = smoothstep(0.01, 0.0, abs(length(uv) - 0.4 * ringScale) - 0.01);
    float bar  = step(abs(uv.y + 0.6), 0.02) * step(abs(uv.x - mix(-1.0, 0.0, slide)), 0.3);
    fragColor = vec4(vec3(ring * ringAlpha + bar), 1.0);
}
```

The same in JS, for Canvas, DOM or WebGL uniforms:

```js
const clamp01 = (x) => Math.min(1, Math.max(0, x));
const seg = (t, start, dur) => clamp01((t - start) / dur);
const outExpo = (x) => (x >= 1 ? 1 : 1 - Math.pow(2, -10 * x));
const inOutCubic = (x) => (x < 0.5 ? 4 * x * x * x : 1 - Math.pow(-2 * x + 2, 3) / 2);

// ?t=2.5 in the URL freezes time at 2.5 s, so any frame can be captured for review.
const frozen = new URLSearchParams(location.search).get("t");
const start = performance.now();
function now() {
  return frozen !== null ? parseFloat(frozen) : (performance.now() - start) / 1000;
}

function draw(t) {
  const ringScale = outExpo(seg(t, 0.0, 0.6));
  const slide = inOutCubic(seg(t, 0.97, 0.48));
  return { ringScale, slide };
}

function loop() {
  draw(now());
  if (frozen === null) requestAnimationFrame(loop);
}
loop();
```

## Don't

- Don't accumulate state per frame (`x += speed`) for choreographed motion; it drifts with
  frame rate and cannot be scrubbed. Accumulation is only for simulations
  ([Feedback](feedback.md), [Particles](particles.md)).
- Don't hard-code a time in two places. Name it once at the top.

## Check

- Rendering a single frame at any `t` gives the same image as reaching it by playback.
- The list of `seg(...)` calls matches the timing chart line by line.

## Next

- [SDF 2D](sdf-2d.md)
- [Overlap](../principle/overlap.md)
