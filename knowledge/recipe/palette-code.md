---
id: palette-code
title: Palette code
summary: Use when coloring in a shader. Hex to linear, OKLab conversion and mixing, gradient maps from palette stops, and constrained cosine palettes.
category: recipe
aliases: [oklab, gradient-map, cosine-palette, color-code, hex]
---
# Palette code

## Rule

Color a scalar field with a gradient map built from the design sheet's palette, mixed in
OKLab. Do not invent colors in code. Cosine palettes are allowed only with the constrained
presets below, because unconstrained ones become rainbows.

## Defaults

| Item | Value |
|---|---|
| gradient map stops | background at 0.0, second at 0.5, main at 0.85, accent above 0.95 |
| mixing space | OKLab |
| cosine palette | one of the measured presets below; check the hue span of any other |

## How

```glsl
vec3 hexToLinear(int h) {
    vec3 c = vec3(float((h >> 16) & 255), float((h >> 8) & 255), float(h & 255)) / 255.0;
    return pow(c, vec3(2.2));
}
// Bjorn Ottosson's OKLab, linear sRGB in and out.
vec3 linearToOklab(vec3 c) {
    float l = 0.4122214708 * c.r + 0.5363325363 * c.g + 0.0514459929 * c.b;
    float m = 0.2119034982 * c.r + 0.6806995451 * c.g + 0.1073969566 * c.b;
    float s = 0.0883024619 * c.r + 0.2817188376 * c.g + 0.6299787005 * c.b;
    vec3 lms = pow(max(vec3(l, m, s), 0.0), vec3(1.0 / 3.0));
    return vec3(
        0.2104542553 * lms.x + 0.7936177850 * lms.y - 0.0040720468 * lms.z,
        1.9779984951 * lms.x - 2.4285922050 * lms.y + 0.4505937099 * lms.z,
        0.0259040371 * lms.x + 0.7827717662 * lms.y - 0.8086757660 * lms.z);
}
vec3 oklabToLinear(vec3 c) {
    float l = c.x + 0.3963377774 * c.y + 0.2158037573 * c.z;
    float m = c.x - 0.1055613458 * c.y - 0.0638541728 * c.z;
    float s = c.x - 0.0894841775 * c.y - 1.2914855480 * c.z;
    vec3 lms = vec3(l, m, s);
    lms = lms * lms * lms;
    return vec3(
         4.0767416621 * lms.x - 3.3077115913 * lms.y + 0.2309699292 * lms.z,
        -1.2684380046 * lms.x + 2.6097574011 * lms.y - 0.3413193965 * lms.z,
        -0.0041960863 * lms.x - 0.7034186147 * lms.y + 1.7076147010 * lms.z);
}
vec3 mixOklab(vec3 a, vec3 b, float t) {
    return oklabToLinear(mix(linearToOklab(a), linearToOklab(b), t));
}

// Gradient map: palette stops at fixed positions. x in 0..1.
vec3 gradientMap(float x) {
    vec3 bg = hexToLinear(0x07131A);
    vec3 second = hexToLinear(0x1E5A6B);
    vec3 mainC = hexToLinear(0x8FD3E0);
    vec3 accent = hexToLinear(0xF2C66D);
    vec3 c = mixOklab(bg, second, smoothstep(0.0, 0.5, x));
    c = mixOklab(c, mainC, smoothstep(0.5, 0.85, x));
    return mixOklab(c, accent, smoothstep(0.95, 1.0, x));
}

// Inigo Quilez cosine palette, with constrained presets (a, b, c, d).
vec3 cosPalette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
    return a + b * cos(6.2831853 * (c * t + d));
}
// Measured presets: with c = 0.5 and d = 0.5, t from 0 to 1 is a dark-to-light ramp.
//   ember: a(0.5,0.25,0.12) b(0.5,0.25,0.12) c(0.5) d(0.5,0.55,0.6)  hue span 24 deg, L 0.04-0.73
//   steel: a(0.35,0.42,0.5) b(0.35,0.4,0.45) c(0.5) d(0.5)           hue span 1 deg,  L 0.06-0.85
//   sea:   a(0.2,0.45,0.5)  b(0.2,0.4,0.4)   c(0.5) d(0.5)           hue span 40 deg, L 0.11-0.83
// With c = 1.0 the phase covers a full cycle and most a, b, d choices sweep 150+ degrees of hue.

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    float x = fragCoord.x / iResolution.x;
    vec3 col = fragCoord.y > 0.5 * iResolution.y
        ? gradientMap(x)
        : cosPalette(x, vec3(0.5, 0.25, 0.12), vec3(0.5, 0.25, 0.12), vec3(0.5), vec3(0.5, 0.55, 0.6));
    fragColor = vec4(pow(max(col, 0.0), vec3(1.0 / 2.2)), 1.0);
}
```

Cosine palette output is already display-like; treat it as sRGB and convert with
`pow(c, vec3(2.2))` before lighting math if you use it inside a linear pipeline.

## Don't

- Don't use `mix(a, b, t)` on sRGB hex values for gradients between distant hues.
- Don't use hue rotation (`hsv`) for animated color cycling; it changes lightness unevenly.
  Rotate OKLab hue instead (rotate the a, b pair).

## Check

- The gradient's midpoints are not gray when the end colors are complementary.
- The accent occupies only the top of the field's range.

## Next

- [Color](../principle/color.md)
- [Palettes](../table/palettes.md)
