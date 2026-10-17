---
id: repetition
title: Repetition and grids
summary: Use when drawing many copies - grids, tiles, rings, radial patterns. Domain repetition with per-cell variation, polar repetition, and limits.
category: recipe
aliases: [grid, tiling, domain-repetition, polar, kaleidoscope, pattern, instancing]
---
# Repetition and grids

## Rule

Repeat the space, not the shape: fold coordinates into a cell, draw one shape, and use the
cell id to vary it. Perfect sameness reads as wallpaper; vary at least one property per cell
(phase, size, color step, presence) with a hash, and keep the variation small.

## Defaults

| Item | Value |
|---|---|
| cells across frame height | 6 to 16 for graphic grids; 30 to 80 for texture |
| per-cell phase offset | `hash(id)` in 0..1 of the cycle, or distance from center for waves |
| per-cell size variation | 0.8 to 1.0 of base |
| cells removed at random | 0 to 30% (gaps make rhythm) |
| polar repetition count | 6, 8, 12 for calm; odd numbers (5, 7, 9) for tension |
| shape size within cell | at most 0.4 of the cell, so neighbors never touch |

## How

```glsl
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
// Square-cell repetition: returns local coords in -0.5..0.5 of the cell, and the cell id.
vec2 repeatGrid(vec2 p, float cell, out vec2 id) {
    id = floor(p / cell + 0.5);
    return (p - id * cell) / cell;
}
// Polar repetition: n copies around the origin. Returns rotated coords and the copy index.
vec2 repeatPolar(vec2 p, float n, out float index) {
    float sector = 6.2831853 / n;
    float a = atan(p.y, p.x) + 0.5 * sector;
    index = floor(a / sector);
    a = mod(a, sector) - 0.5 * sector;
    return length(p) * vec2(cos(a), sin(a));
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float px = 2.0 / iResolution.y;
    vec2 id;
    float cell = 0.16;
    vec2 q = repeatGrid(uv, cell, id);
    float h = hash12(id);
    float wave = 0.5 + 0.5 * sin(iTime * 1.2 - length(id) * 0.6);   // ripple from center
    float r = 0.4 * mix(0.8, 1.0, h) * mix(0.3, 1.0, wave);
    float keep = step(0.2, h);                                        // remove 20%
    float d = (length(q) - r) * cell;                                 // back to uv units
    float dots = keep * clamp(0.5 - d / px, 0.0, 1.0);
    fragColor = vec4(pow(vec3(dots * 0.9), vec3(1.0 / 2.2)), 1.0);
}
```

Distances computed in cell space must be multiplied by the cell size before anti-aliasing,
as above, or edges blur or alias when the cell size changes.

## Don't

- Don't let shapes cross cell borders; they get cut at the edge. Keep size at most 0.4 cell,
  or check the 4 neighbor cells.
- Don't animate all cells with the same phase: [mechanical-stagger](../rule/mechanical-stagger.md).
- Don't fill every cell when the grid is the subject; gaps give it rhythm.

## Check

- No visible seams at cell borders.
- Squinting, the grid shows a larger pattern (wave, gradient, cluster), not uniform gray.

## Next

- [Feedback](feedback.md)
- [Op-art](../look/op-art.md)
