---
id: shader-frame
title: Shader frame (start every shader here)
summary: Use when writing any fragment shader. A template with aspect-correct coordinates, pixel-width anti-aliasing, linear color, vignette, grain and dither already right.
category: recipe
aliases: [shader-basics, template, boilerplate, glsl, shadertoy]
---
# Shader frame

## Rule

Copy this template and build inside it. It fixes the five defects that make shaders look
amateur: stretched coordinates, aliased edges, sRGB math, banding, and dead-flat pixels.
Conventions are Shadertoy's (`mainImage`, `iTime`, `iResolution`); port by renaming.

## Defaults

| Item | Value |
|---|---|
| coordinates | `uv` y from -1 to 1, x scaled by aspect, origin at center |
| pixel size in uv | `px = 2.0 / iResolution.y` |
| edge width | 1 px (`fill(d, px)`); 1.5 px for moving thin lines |
| color constants | written as sRGB hex, converted to linear once |
| vignette | 0.25 |
| grain | 0.03 |
| dither | 1/255 |

## How

```glsl
// ---- palette: sRGB hex in, linear out ----
vec3 hexToLinear(int h) {
    vec3 c = vec3(float((h >> 16) & 255), float((h >> 8) & 255), float(h & 255)) / 255.0;
    return pow(c, vec3(2.2));
}
#define BG     hexToLinear(0x0E0F13)
#define MAIN   hexToLinear(0xE9E6DF)
#define ACCENT hexToLinear(0xFF5A36)

const float VIGNETTE = 0.25;
const float GRAIN    = 0.03;

float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

// Anti-aliased coverage of a signed distance d (negative inside), px = pixel size.
float fill(float d, float px) { return clamp(0.5 - d / px, 0.0, 1.0); }
// Anti-aliased stroke of width w centered on the d = 0 contour.
float stroke(float d, float w, float px) { return fill(abs(d) - 0.5 * w, px); }

vec3 linearToSrgb(vec3 c) { return pow(max(c, 0.0), vec3(1.0 / 2.2)); }

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float px = 2.0 / iResolution.y;
    float t = iTime;

    // ---- scene, in linear RGB ----
    vec3 col = BG;
    float d = length(uv - vec2(0.0, 0.05)) - 0.45;          // optical center: slightly up
    col = mix(col, MAIN, fill(d, px));
    col = mix(col, ACCENT, stroke(d - 0.08, 0.012, px) * (0.5 + 0.5 * sin(t)));

    // ---- finish ----
    col *= 1.0 - VIGNETTE * smoothstep(0.4, 1.8, length(uv));
    col = linearToSrgb(col);
    col += (hash12(fragCoord + fract(t) * 917.0) - 0.5) * GRAIN;  // grain, new each frame
    col += (hash12(fragCoord.yx + 71.0) - 0.5) / 255.0;            // dither, last
    fragColor = vec4(col, 1.0);
}
```

Output target: Shadertoy and a plain WebGL canvas expect sRGB values, so the template
converts. If you render into an sRGB framebuffer (WebGPU `*-srgb` formats, `GL_FRAMEBUFFER_SRGB`),
remove `linearToSrgb`: the hardware converts, and doing it twice washes out the image.

## Don't

- Don't use `fragCoord / iResolution.xy` as your shape space: circles become ellipses.
- Don't use `step` for edges: [aliasing](../rule/aliasing.md).
- Don't mix colors after `linearToSrgb`, except grain and dither.
- Don't remove the dither line: [banding](../rule/banding.md).

## Check

- A circle of radius 0.45 is round at 16:9 and 9:16.
- Zoomed 4x, edges have a one-pixel soft ramp.

## Next

- [Timeline](timeline.md)
- [SDF 2D](sdf-2d.md)
- [Post chain](post-chain.md)
