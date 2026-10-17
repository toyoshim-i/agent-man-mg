---
id: example-vj-loop
title: Example - VJ loop (deep techno, GLSL)
summary: Use when making a beat-synced VJ loop or ambient live visual. A complete worked example of a seamless 8-bar hypnotic loop with kick response, breath and accent.
category: example
aliases: [vj-loop, techno-visual, live-loop, beat-loop]
---
# Example: VJ loop

Every step of [the procedure](../start/start.md), with its written output.

## 1. Brief

```text
purpose:     keep a deep techno room locked in; hypnotic, never distracting
audience:    club, projector in haze, 2 to 4 hour set
medium:      GLSL fragment shader, one layer (base role)
format:      1920x1080, 60 fps
duration:    loop 8 bars at 128 BPM = 15.0 s
content:     none
music:       128 BPM, four-on-the-floor
mood words:  hypnotic, deep, dark
must avoid:  full-screen flashes, text, fast motion
done when:   t = 0 and t = 15 s are identical; kick response is visible but small
```

## 2. Concept

Purpose as a question: "How does a techno room stay locked in for three hours without the
visuals tiring it?" Three ideas from three families:

```text
idea:          see-the-sound (sound and image)
then asked:    how can image and sound be one score?
now asks:      every visualizer pulses to everything; what does it mean to listen?
became free:   audio-reactive pulsing
became scarce: deciding what NOT to sync; restraint
opening:       one element listens, everything else holds still

idea:          ma (time and meaning)
then asked:    what if the empty interval is the expression?
now asks:      a club wall full of motion all night; where does the room rest?
became free:   filling every second
became scarce: darkness and stillness as events
opening:       the breath: rings dim for two beats before every fourth bar

idea:          math-is-the-image (constraint and medium)
then asked:    what if a formula made visible is the picture?
now asks:      generated tunnels look rich without meaning; what does one formula give?
became free:   rich 3D spectacle
became scarce: one legible rule held for hours
opening:       points in harmonic ratios that align into a figure on every phrase
```

| Concept | Purpose | Now | Glance | Build | Mood | Total |
|---|---|---|---|---|---|---|
| "One ring of light breathes on the kick, its echoes sinking into fog; everything else is still" | 2 | 2 | 2 | 2 | 2 | 10 |
| "Harmonic points scatter and align into a figure on every phrase" | 1 | 1 | 1 | 2 | 2 | 7 |
| "The wall goes dark; one line of light crosses it once per phrase" | 1 | 2 | 2 | 2 | 1 | 8 |

Chosen: the ring, with the breath from *ma* built in. The dark wall is kept for the set:
use it once, before the biggest drop of the night.

## 3. Design sheet

```text
look:      op-art crossed with organic fog
background #07131A    main #8FD3E0    accent #F2C66D (one ring, on every 4-bar downbeat)
shapes:    rings only; echo rings receding in log space, one per bar
motion:    periodic; kick envelope exp(-6 * beatPhase); rotation 1/6 turn per loop
speed:     factor 1.4; echoes move one step per bar
density:   0.25
texture:   fog from looping noise, glow on the main ring, Reinhard tonemap, dither
camera:    static
```

## 4. Structure

Energy over 4 bars, repeated twice: bars 1 to 3 at 4, last 2 beats of bar 4 dip to 2 (breath),
downbeat of the next bar at 6 (accent ring). Every quantity is periodic in the beat, the bar or
the loop, so the loop is seamless by construction.

```text
beat-level   kick      main ring radius +4%, glow x(0.4 -> 1.0)    exp(-6 * phase)
beat-level   off-beat  echoes brighten slightly                    exp(-8 * phase')
bar-level    echoes    move inward one ring per bar                linear in bar phase
4-bar        breath    rings dim to 50% on beats 15-16 of 16       smoothstep
4-bar        accent    one ring in accent, decays over 2 beats     exp(-3 * beats)
loop-level   rotation  6 gaps in the main ring, 1/6 turn per loop  linear
loop-level   fog       noise on a circle, period 15 s              loopNoise
```

## 5. Build

```glsl
vec3 hexToLinear(int h) {
    vec3 c = vec3(float((h >> 16) & 255), float((h >> 8) & 255), float(h & 255)) / 255.0;
    return pow(c, vec3(2.2));
}
#define BG     hexToLinear(0x07131A)
#define MAIN   hexToLinear(0x8FD3E0)
#define ACCENT hexToLinear(0xF2C66D)

const float BPM  = 128.0;
const float BARS = 8.0;
const float LOOP = BARS * 4.0 * 60.0 / BPM;   // 15.0 s
const float R0   = 0.55;                       // main ring radius
const float STEP = 0.72;                       // echo radius ratio per ring

float hash31(vec3 p) {
    p = fract(p * 0.1031);
    p += dot(p, p.zyx + 31.32);
    return fract((p.x + p.y) * p.z);
}
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
float vnoise3(vec3 p) {
    vec3 i = floor(p);
    vec3 f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(mix(hash31(i), hash31(i + vec3(1, 0, 0)), f.x),
                   mix(hash31(i + vec3(0, 1, 0)), hash31(i + vec3(1, 1, 0)), f.x), f.y),
               mix(mix(hash31(i + vec3(0, 0, 1)), hash31(i + vec3(1, 0, 1)), f.x),
                   mix(hash31(i + vec3(0, 1, 1)), hash31(i + vec3(1, 1, 1)), f.x), f.y), f.z);
}
// Three-octave fog that loops every LOOP seconds (noise sampled on a circle in x and z).
float fog(vec2 uv, float t) {
    float a = 6.2831853 * t / LOOP;
    vec3 q = vec3(uv * 1.3, 0.0) + 0.9 * vec3(cos(a), 0.0, sin(a));
    return 0.55 * vnoise3(q) + 0.3 * vnoise3(q * 2.03 + 7.1) + 0.15 * vnoise3(q * 4.01 + 3.7);
}
// Anti-aliased line at the integer crossings of x, width w in x units.
float lines(float x, float w) {
    float u = abs(fract(x + 0.5) - 0.5);
    return clamp((w - u) / fwidth(x) + 0.5, 0.0, 1.0);
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float t = mod(iTime, LOOP);
    float beat = t * BPM / 60.0;
    float phase = fract(beat);
    float kick = exp(-6.0 * phase);
    float off = exp(-8.0 * fract(phase + 0.5));
    float b16 = mod(beat, 16.0);                               // position in 4 bars
    float breath = 1.0 - 0.5 * smoothstep(14.0, 15.5, b16);
    float accent = exp(-1.5 * b16);
    float r = length(uv);

    // Echo rings inside the main ring: ring n sits at radius R0 * STEP^n (log-spaced, like
    // depth). Subtracting the bar phase sinks every ring one step inward per bar.
    float depth = log(max(r, 1e-4) / R0) / log(STEP);          // 0 at the main ring
    float k = depth - fract(beat / 4.0);
    float inside = smoothstep(R0 * 0.98, R0 * 0.88, r) * smoothstep(9.0, 6.0, depth);
    float echoes = lines(k, 0.035) * exp(-0.45 * depth) * (0.25 + 0.1 * off) * inside;
    float accentRing = lines(k, 0.05) * step(abs(k - 1.0), 0.5) * accent * inside;

    // Main ring: breathes with the kick, six gaps rotating 1/6 turn per loop.
    float rr = R0 * (1.0 + 0.04 * kick);
    float a = atan(uv.y, uv.x) / 6.2831853 + t / LOOP / 6.0;
    float gaps = smoothstep(0.04, 0.06, abs(fract(a * 6.0) - 0.5));
    float dRing = abs(r - rr);
    float ring = clamp((0.006 - dRing) / fwidth(r) + 0.5, 0.0, 1.0) * gaps;
    // Gaps cut the line and the glow next to it, but not the far glow: otherwise they
    // extend into the fog as dark spokes.
    float glow = exp(-dRing * 14.0) * (0.4 + 0.6 * kick) * mix(1.0, gaps, exp(-dRing * 40.0));

    float haze = fog(uv, t);
    vec3 col = BG * (0.7 + 0.6 * haze);
    col += MAIN * (ring * 1.4 + glow * 0.5 + echoes) * breath;
    col += ACCENT * accentRing * 1.2;
    col += MAIN * 0.05 * haze * exp(-dRing * 3.0);              // light scattered in the fog
    col = col / (1.0 + col);                                     // Reinhard
    col = pow(col, vec3(1.0 / 2.2));
    col += (hash12(fragCoord) - 0.5) / 255.0;
    fragColor = vec4(col, 1.0);
}
```

To drive it live, replace `kick` with the kick envelope from
[Audio reactive](../recipe/audio-reactive.md) and `beat` with the clock from [Sync](../vj/vj-sync.md).

## 6. Review

| Check | Expected |
|---|---|
| Seam | every term is periodic in beat, bar, 4 bars or LOOP, so t = 0 equals t = 15 s |
| Flashes | the kick changes a ring and its glow, well under 25% of the screen |
| One star | the main ring; echoes are at 25% brightness and fade with depth |
| Energy | breath on beats 15 to 16, accent on the downbeat: one event per 4 bars |
| Projection | empty space is near black; the ring is at L 0.8 and survives haze |

Found when rendered: the first draft multiplied the whole glow by `gaps`, so the six gaps ran
across the halo as dark spokes reaching into the fog. They read as an artifact, not a design.
The fix limits the gaps to the line and its near glow.

## Next

- [Kinetic quote example](example-kinetic-quote.md)
- [VJ set](../vj/vj-set.md)
