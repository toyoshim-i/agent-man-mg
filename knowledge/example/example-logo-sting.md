---
id: example-logo-sting
title: Example - logo sting (fintech, GLSL)
summary: Use when making a logo sting, logo reveal or title resolve. A complete worked example from brief to finished shader, including the review rounds.
category: example
aliases: [logo-reveal, sting, logo-animation, ident]
---
# Example: logo sting

Every step of [the procedure](../start/start.md) is shown with its written output. Copy the
shape of each step, not the content.

## 1. Brief

```text
purpose:     viewers feel "this company is precise and safe with money"
audience:    start of product videos and conference talks; big screens and phones
medium:      GLSL fragment shader (assumed)
format:      1920x1080, 16:9, 60 fps (assumed)
duration:    5 s
content:     the mark only (a ring crossed by a vertical "north line"); wordmark added in edit
music:       none
mood words:  precise, trustworthy, calm
must avoid:  anything playful or glitchy
done when:   the mark is fully formed and still by t = 3.0 s and readable at 160 px wide
```

## 2. Concept

Purpose as a question: "How does a money company feel precise without feeling cold?"
Three ideas from three families:

```text
idea:          identity-as-system (rules and systems)
then asked:    how can a brand stay recognizable while its picture changes?
now asks:      templates animate every logo smoothly; what makes motion mean this brand?
became free:   polished logo animation
became scarce: motion that states the company's own principle
opening:       the mark is the end state of a rule: order emerging from scattered parts

idea:          see-the-sound (sound and image)
then asked:    how can image and sound be one score?
now asks:      with no music, can motion alone carry a calm, steady pulse?
became free:   pulsing, reactive motion
became scarce: one steady rhythm held with restraint
opening:       ink settles into the mark with a slow breathing rhythm

idea:          in-camera-truth (body, hand and character)
then asked:    how can an image prove it really happened?
now asks:      generated images look like anything; how does a sting feel trustworthy?
became free:   any surface or material, rendered convincingly
became scarce: evidence of a real, checkable process
opening:       a mechanical vault door turns and locks, every part visible
```

| Concept | Purpose | Now | Glance | Build | Mood | Total |
|---|---|---|---|---|---|---|
| "Scattered points lock into a lattice, and the lattice is the mark" | 2 | 2 | 2 | 2 | 2 | 10 |
| "Ink settles into a still pool shaped like the mark" | 1 | 1 | 2 | 1 | 1 | 6 |
| "A vault door turns and locks into the mark" | 2 | 1 | 1 | 1 | 1 | 6 |

Chosen: the lattice. The vault is kept only as the feel of the final lock: no overshoot.

## 3. Design sheet

```text
look:      swiss, dark variant
background #0B1B2B (0.22 0.04 250)    main #F4F7FA (0.97 0.01 248)
second     #2F6FB2 (0.53 0.12 252), unused    accent #3DDC97 (0.80 0.16 160), north tip only
type:      none in shader
shapes:    dots, ring, bar; ring stroke 0.09 uv
motion:    outExpo arrivals, inOutCubic forms, no overshoot; speed factor 1.0
density:   0.3
texture:   grain 0, vignette 0.15 as a soft radial background, dither
camera:    slow push 3% during the hold
```

## 4. Structure

Energy: setup 3, build 6, breath 1, peak 9, release 4.

```text
t_start  dur    element      property           ease
0.00     0.60   dots         radius 0 -> small  outExpo, jittered 0.3 s
0.80     0.60   dots         scatter -> grid    outExpo, staggered by distance, spread 0.5
2.00     0.12   all          (breath)           hold
2.12     0.50   dots         merge into mark    inOutCubic, from center out
2.55     0.40   mark         dots -> solid      inOutCubic, dots shrink as it appears
2.80     0.40   north tip    main -> accent     outExpo, top down
2.90     2.10   camera       push 1.00 -> 1.03  inOutSine
```

## 5. Build

```glsl
vec3 hexToLinear(int h) {
    vec3 c = vec3(float((h >> 16) & 255), float((h >> 8) & 255), float(h & 255)) / 255.0;
    return pow(c, vec3(2.2));
}
#define BG     hexToLinear(0x0B1B2B)
#define MAIN   hexToLinear(0xF4F7FA)
#define ACCENT hexToLinear(0x3DDC97)

const float T_DOTS_IN = 0.00;
const float D_DOTS_IN = 0.60;
const float T_GATHER  = 0.80;
const float D_GATHER  = 0.60;
const float S_GATHER  = 0.50;
const float T_FORM    = 2.12;
const float D_FORM    = 0.50;
const float T_SOLID   = 2.55;
const float D_SOLID   = 0.40;
const float T_ACCENT  = 2.80;
const float D_ACCENT  = 0.40;
const float T_PUSH    = 2.90;
const float DURATION  = 5.00;
const float CELL      = 0.09;
const float FIELD     = 0.95;   // dots exist inside this radius

float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
vec2 hash22(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * vec3(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.xx + p3.yz) * p3.zy);
}
float seg(float t, float s, float d) { return clamp((t - s) / d, 0.0, 1.0); }
float outExpo(float x) { return x >= 1.0 ? 1.0 : 1.0 - pow(2.0, -10.0 * x); }
float inOutCubic(float x) { return x < 0.5 ? 4.0 * x * x * x : 1.0 - pow(-2.0 * x + 2.0, 3.0) * 0.5; }
float inOutSine(float x) { return -(cos(3.14159265 * x) - 1.0) * 0.5; }

float sdMark(vec2 p) {
    float ring = abs(length(p) - 0.42) - 0.045;
    vec2 q = abs(p) - vec2(0.045, 0.62);
    float bar = length(max(q, 0.0)) + min(max(q.x, q.y), 0.0);
    return min(ring, bar);
}

// Coverage of the dot field at uv: each dot flies from a scattered start to its grid home,
// then grows if its home is on the mark, or vanishes if not, and finally shrinks away as
// the solid mark fades in under it.
float dots(vec2 uv, float t, float px, float solid) {
    float cov = 0.0;
    vec2 base = floor(uv / CELL + 0.5);
    for (int j = -1; j <= 1; j++) {
        for (int i = -1; i <= 1; i++) {
            vec2 id = base + vec2(float(i), float(j));
            vec2 home = id * CELL;
            float dist = length(home) / FIELD;
            if (dist > 1.0) continue;
            float h = hash12(id);
            vec2 scatter = (hash22(id) - 0.5) * CELL * 1.6;
            float g = outExpo(seg(t, T_GATHER + S_GATHER * dist * mix(0.7, 1.3, h), D_GATHER));
            vec2 pos = home + scatter * (1.0 - g);
            float onMark = step(sdMark(home), CELL * 0.25);
            float form = inOutCubic(seg(t, T_FORM + 0.15 * dist, D_FORM));
            float r = CELL * 0.16 * outExpo(seg(t, T_DOTS_IN + 0.3 * h, D_DOTS_IN));
            r = mix(r, onMark * CELL * 0.58, form) * (1.0 - solid);   // condense into the mark
            cov = max(cov, clamp(0.5 - (length(uv - pos) - r) / px, 0.0, 1.0));
        }
    }
    return cov;
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float t = mod(iTime, DURATION);                       // looped for preview only
    uv /= 1.0 + 0.03 * inOutSine(seg(t, T_PUSH, DURATION - T_PUSH));
    float px = 2.0 / iResolution.y;

    float solid = inOutCubic(seg(t, T_SOLID, D_SOLID));
    float cov = max(dots(uv, t, px, solid), clamp(0.5 - sdMark(uv) / px, 0.0, 1.0) * solid);

    float tipEdge = mix(0.62, 0.49, outExpo(seg(t, T_ACCENT, D_ACCENT)));
    float tip = smoothstep(tipEdge - px, tipEdge + px, uv.y) * solid;

    vec3 col = BG * mix(1.15, 0.85, smoothstep(0.0, 1.6, length(uv)));
    col = mix(col, mix(MAIN, ACCENT, tip), cov);
    col = pow(col, vec3(1.0 / 2.2));
    col += (hash12(fragCoord) - 0.5) / 255.0;            // dither
    fragColor = vec4(col, 1.0);
}
```

## 6. Review

Rendered as a contact sheet at t = 0.3, 1.0, 1.2, 1.4, 2.05, 2.3, 2.5, 2.7, 3.0, 4.9.

Round 1:

| Check | Result | Action |
|---|---|---|
| Poster (t = 2.6) | the merged dots formed a shape twice as thick as the final mark | `onMark` threshold from 0.5 to 0.25 cells, merged radius 0.58 cells |
| Popping / ghost (t = 2.8) | crossfading dots into a different shape left a gray halo | dots shrink to 0 while the solid mark fades in (`max`, not `mix`) |
| One star | the solid fade began before the outer dots finished merging | `T_SOLID` moved from 2.45 to 2.55, after the merge |

Round 2: the dot shape matches the mark, the mark condenses cleanly, still by 2.95 s.
Thumbnail at 160 px: ring, line and green tip read. Grayscale: the mark is the only
high-contrast region. Stopped.

## Next

- [VJ loop example](example-vj-loop.md)
- [Review](../process/6-review.md)
