---
id: raymarch
title: Raymarching 3D
summary: Use when a shader needs 3D forms. A minimal sphere-tracer with camera, normals, soft shadows, ambient occlusion, three-light shading and fog, with budgets.
category: recipe
aliases: [raymarching, sphere-tracing, 3d, sdf-3d, lighting]
---
# Raymarching 3D

## Rule

Light it like a photographer: one key light that casts soft shadows, a sky fill from above,
a rim from behind, ambient occlusion in the creases, and fog that pushes the background back.
Unlit or single-light raymarching looks like a tech demo.

## Defaults

| Item | Value |
|---|---|
| max steps | 96 to 128 (64 for live at 4K) |
| hit epsilon | `0.001 * t` (relative to distance); out of steps below max distance is a hit |
| max distance | 20 to 40 |
| focal length | 1.5 to 2.5 (lower is wider) |
| key light | warm, intensity 1.5 to 2.5, soft shadow k 8 to 16 |
| sky fill | cool, 0.2 to 0.4, weighted by `0.5 + 0.5 * n.y` |
| rim | 0.2 to 0.5, `pow(1 - dot(n, -rd), 3)` |
| ambient occlusion | 5 samples, step 0.03 to 0.12 |
| fog | `exp(-0.04 to -0.12 * t)` toward the background color |

## How

```glsl
mat2 rot(float a) { float c = cos(a); float s = sin(a); return mat2(c, s, -s, c); }
float sdBox(vec3 p, vec3 b) {
    vec3 q = abs(p) - b;
    return length(max(q, 0.0)) + min(max(q.x, max(q.y, q.z)), 0.0);
}
float smin(float a, float b, float k) {
    float h = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}
float map(vec3 p) {
    vec3 q = p;
    q.xz = rot(iTime * 0.2) * q.xz;
    float obj = smin(sdBox(q, vec3(0.55)) - 0.05,
                     length(p - vec3(0.0, 0.6 * sin(iTime * 0.7), 0.0)) - 0.5, 0.25);
    return min(obj, p.y + 1.0);                                   // floor
}
vec3 calcNormal(vec3 p) {
    const vec2 k = vec2(1.0, -1.0);
    const float h = 0.0005;
    return normalize(k.xyy * map(p + k.xyy * h) + k.yyx * map(p + k.yyx * h) +
                     k.yxy * map(p + k.yxy * h) + k.xxx * map(p + k.xxx * h));
}
float softShadow(vec3 ro, vec3 rd, float k) {
    float res = 1.0;
    float t = 0.02;
    for (int i = 0; i < 48; i++) {
        float h = map(ro + rd * t);
        res = min(res, k * h / t);
        t += clamp(h, 0.02, 0.2);
        if (res < 0.001 || t > 10.0) break;
    }
    return clamp(res, 0.0, 1.0);
}
float ambientOcclusion(vec3 p, vec3 n) {
    float occ = 0.0;
    float sca = 1.0;
    for (int i = 0; i < 5; i++) {
        float h = 0.03 + 0.09 * float(i);
        occ += (h - map(p + h * n)) * sca;
        sca *= 0.9;
    }
    return clamp(1.0 - 2.5 * occ, 0.0, 1.0);
}

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 uv = (2.0 * fragCoord - iResolution.xy) / iResolution.y;
    float a = 0.4 + 0.1 * iTime;
    vec3 ro = vec3(3.2 * cos(a), 1.1, 3.2 * sin(a));
    vec3 ww = normalize(vec3(0.0) - ro);
    vec3 uu = normalize(cross(ww, vec3(0.0, 1.0, 0.0)));
    vec3 vv = cross(uu, ww);
    vec3 rd = normalize(uv.x * uu + uv.y * vv + 1.8 * ww);

    vec3 bg = vec3(0.02, 0.025, 0.035);
    // A ray that runs out of steps before converging (grazing the floor near the horizon)
    // still counts as a hit unless it left the scene; otherwise the horizon gets dotted holes.
    const float TMAX = 30.0;
    float t = 0.0;
    for (int i = 0; i < 128; i++) {
        float h = map(ro + rd * t);
        if (h < 0.001 * t || t > TMAX) break;
        t += h;
    }
    vec3 col = bg;
    if (t < TMAX) {
        vec3 p = ro + rd * t;
        vec3 n = calcNormal(p);
        vec3 albedo = p.y < -0.99 ? vec3(0.05) : vec3(0.7, 0.68, 0.64);   // dark floor: subject is the star
        vec3 l = normalize(vec3(0.6, 0.8, -0.4));
        float key = clamp(dot(n, l), 0.0, 1.0) * softShadow(p + n * 0.002, l, 12.0);
        float sky = 0.5 + 0.5 * n.y;
        float rim = pow(1.0 - clamp(dot(n, -rd), 0.0, 1.0), 3.0);
        float ao = ambientOcclusion(p, n);
        vec3 light = 2.0 * key * vec3(1.0, 0.85, 0.7)
                   + 0.3 * sky * ao * vec3(0.5, 0.65, 1.0)
                   + 0.35 * rim * ao * vec3(1.0);
        col = albedo * light;
        col = mix(bg, col, exp(-0.12 * t));                       // fog
    }
    col = col / (1.0 + col);                                      // Reinhard tonemap
    fragColor = vec4(pow(col, vec3(1.0 / 2.2)), 1.0);
}
```

## Don't

- Don't color by normal (`n * 0.5 + 0.5`) in a final piece; it is a debug view.
- Don't step past surfaces: if you deform space (twist, bend), multiply each step by 0.5 to 0.8.
- Don't light from the camera position; shapes go flat.

## Check

- A gray material reads as 3D in grayscale: key, shadow and occlusion visible.
- The frame holds 60 fps at the target resolution; if not, lower steps first, then resolution.

## Next

- [Camera](../principle/camera.md)
- [Post chain](post-chain.md)
