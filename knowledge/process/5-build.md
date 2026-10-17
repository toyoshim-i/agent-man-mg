---
id: build
title: Step 5 - Build
summary: Use when the timing chart is written and you start coding. The build order and code layout that keep a piece tunable.
category: process
aliases: [implement, code, step-5]
---
# Step 5: Build

## Rule

Build in five passes, and look at the result after each one. Never write the whole piece in
one go: a defect found in pass 5 is usually caused in pass 1, and by then it is buried.

| Pass | Build | Look for |
|---|---|---|
| 1. Still frame | the peak frame, no time | composition, value contrast, palette |
| 2. Primary motion | the timing chart's primary events | timing, easing, readable order |
| 3. Secondary motion | offsets, follow-through, ambient drift | life without clutter |
| 4. Finish | AA, color pipeline, grain, dither, vignette, bloom | expensive look, no banding |
| 5. Audio / live | audio mapping, live controls | response without jitter |

Pass 1 matters most. If the still peak frame is not a good poster, motion will not save it.

## Code layout

Put every number from the design sheet and timing chart in one block at the top, named.
Tuning then means editing that block, not hunting through the code.

```glsl
vec3 hexToLinear(int h) {
    vec3 c = vec3(float((h >> 16) & 255), float((h >> 8) & 255), float(h & 255)) / 255.0;
    return pow(c, vec3(2.2));
}
// ---- design sheet (palette ink-night) ----
#define BG     hexToLinear(0x0E0F13)   // OKLCH L 0.17
#define MAIN   hexToLinear(0xE9E6DF)   // OKLCH L 0.93
#define ACCENT hexToLinear(0xFF5A36)   // at most 10% of the frame
const float GRAIN = 0.04;
// ---- timing chart (seconds) ----
const float T_RING_IN  = 0.00;
const float D_RING_IN  = 0.60;
const float T_BREATH   = 0.90;
const float T_LOGO     = 0.98;
const float DURATION   = 4.00;

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float t = mod(iTime, DURATION);
    float px = 2.0 / iResolution.y;
    float ringIn = clamp((t - T_RING_IN) / D_RING_IN, 0.0, 1.0);   // eased in pass 2
    float disc = clamp(0.5 - (length(uv) - 0.5) / px, 0.0, 1.0);
    vec3 col = mix(BG, MAIN, ringIn * disc);
    fragColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
```

## How

1. Start from the [Shader frame](../recipe/shader-frame.md) template (or the HTML template in
   [Kinetic type code](../recipe/kinetic-type-code.md)). It already has the color pipeline and AA.
2. Pass 1: set `t` to the peak time as a constant. Build the frame. Run the still-frame checks
   in [Review](6-review.md).
3. Pass 2: wire `t` to time. Implement the chart with [Timeline](../recipe/timeline.md)
   helpers, one event per line, in chart order.
4. Pass 3: add secondary motion from [Overlap](../principle/overlap.md). Amplitudes are a
   fraction of the primary: 0.1 to 0.3.
5. Pass 4: apply [Finish](../principle/finish.md) in the order given in
   [Post chain](../recipe/post-chain.md).
6. Pass 5: [Audio reactive](../recipe/audio-reactive.md) and [Live controls](../vj/vj-controls.md).

## Don't

- Don't write effects that are not in the design sheet. Add them to the sheet first, or drop them.
- Don't scatter magic numbers. A number used twice is a named constant.
- Don't tune by changing many values at once. Change one, look, keep or revert.

## Check

- The peak frame alone passes the still-frame checks.
- Every timing chart line maps to exactly one line of timing code.

## Next

- [Review](6-review.md)
