---
id: glitch
title: Glitch
summary: Use when a piece calls for digital corruption. Block displacement, RGB split and scanlines gated into short bursts, so glitch stays an accent.
category: recipe
aliases: [datamosh, rgb-split, corruption, digital-error, distortion]
---
# Glitch

## Rule

Glitch is punctuation. It happens in bursts of 2 to 6 frames, on accents (beats, cuts, hits),
and is absent most of the time. A constant glitch is just noise. Corrupt coordinates in
blocks aligned to a coarse grid; real digital errors are blocky, not wavy.

## Defaults

| Parameter | Value |
|---|---|
| frames with glitch | 5 to 15% overall; up to 30% in a peak section |
| burst length | 2 to 6 frames (0.03 to 0.1 s) |
| decision rate | 8 to 16 per second (the burst slot length) |
| block grid | 4 to 12 columns, 12 to 40 rows |
| block shift | 2 to 20% of frame width, only in 20 to 40% of blocks |
| RGB split | 0.002 to 0.01 uv, horizontal |
| scanlines | 0.03 to 0.08 darkening, period 2 to 4 px |

## How

```glsl
float hash11(float n) { return fract(sin(n * 127.1) * 43758.5453); }
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
// Burst gate: on in about 12% of 1/12 s slots, with a random strength.
float glitchAmount(float t) {
    float slot = floor(t * 12.0);
    return step(0.88, hash11(slot)) * mix(0.4, 1.0, hash11(slot + 7.0));
}
vec3 scene(vec2 uv) {
    float px = 2.0 / iResolution.y;
    float bars = clamp(0.5 - (abs(fract(uv.x * 2.0 + 0.5) - 0.5) - 0.2) / px, 0.0, 1.0);
    float disc = clamp(0.5 - (length(uv) - 0.55) / px, 0.0, 1.0);
    return mix(vec3(0.02), vec3(0.85), abs(bars - disc));
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float g = glitchAmount(iTime);
    float slot = floor(iTime * 12.0);

    vec2 block = floor((fragCoord / iResolution.xy) * vec2(8.0, 24.0));
    float r = hash12(block + slot);
    vec2 duv = uv;
    duv.x += g * step(0.7, r) * (r - 0.85) * 0.8;                 // shift 30% of blocks

    float split = g * 0.008;
    vec3 col = vec3(scene(duv + vec2(split, 0.0)).r,
                    scene(duv).g,
                    scene(duv - vec2(split, 0.0)).b);
    col *= 1.0 - g * 0.06 * step(0.5, fract(fragCoord.y / 3.0)); // scanlines only in bursts
    fragColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
```

To lock bursts to music, replace `glitchAmount` with the kick or snare envelope from
[Audio reactive](audio-reactive.md), gated by a random chance per beat.

## Don't

- Don't run RGB split or wobble continuously: [over-effected](../rule/over-effected.md).
- Don't glitch text that must be read during its hold.
- Don't make full-frame white or inverted flashes part of the glitch at more than 3 per
  second: [strobe-risk](../rule/strobe-risk.md).

## Check

- Count frames with glitch over 10 s: 5 to 15% outside the peak.
- Every burst lasts 6 frames or less.

## Next

- [Audio reactive](audio-reactive.md)
- [CRT and VHS look](../look/crt-vhs.md)
