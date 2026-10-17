---
id: particles
title: Particles and flow fields
summary: Use when you need dust, sparks, swarms or flowing lines. A curl-noise flow field that never clumps, size and opacity distributions, trails, and counts.
category: recipe
aliases: [particle-system, flow-field, curl-noise, swarm, dust, sparks]
---
# Particles and flow fields

## Rule

Particles look good when their distribution is uneven: many small and faint, few large and
bright. Move them with a divergence-free (curl) field so they swirl instead of clumping into
lines. Give each one a life so the field keeps renewing.

## Defaults

| Parameter | Value |
|---|---|
| count, Canvas 2D at 1080p | 800 to 3000 |
| count, GPU | 10k to 1M |
| size distribution | `min + (max - min) * r^4` with r uniform: mostly small |
| size range | 0.5 to 4 px for dust; 2 to 12 px for bokeh |
| opacity | 0.2 to 0.9, tied to size or life (fade in 10%, out 30% of life) |
| speed | 0.02 to 0.1 frame heights per second (ambient) |
| life | 4 to 12 s, randomized per particle |
| trails | redraw background at alpha 0.05 to 0.15 per frame, scaled by frame time |
| blending | additive (`lighter`) for light; normal for ink and dust |

## How

Canvas 2D, frame-rate independent, with an analytic curl field:

```js
const canvas = document.querySelector("canvas");
const ctx = canvas.getContext("2d");
const W = (canvas.width = 1920);
const H = (canvas.height = 1080);
const N = 2000;

// Stream function psi(x, y, t); velocity is its curl: (d psi/dy, -d psi/dx).
function curl(x, y, t) {
  const a = 1.7 * x + 0.1 * t;
  const b = 2.3 * y - 0.13 * t;
  const c = 3.1 * x + 1.9 * y + 0.07 * t;
  const dPsiDx = 1.7 * Math.cos(a) * Math.cos(b) + 0.5 * 3.1 * Math.cos(c);
  const dPsiDy = -2.3 * Math.sin(a) * Math.sin(b) + 0.5 * 1.9 * Math.cos(c);
  return [dPsiDy, -dPsiDx];
}

function spawn(p) {
  p.x = Math.random() * (W / H); // x in 0..aspect, y in 0..1 (frame-height units)
  p.y = Math.random();
  p.size = 0.5 + 3.5 * Math.pow(Math.random(), 4);
  p.life = 4 + 8 * Math.random();
  p.age = 0;
  return p;
}
const ps = Array.from({ length: N }, () => spawn({}));

let last = performance.now();
function frame(now) {
  const dt = Math.min(0.05, (now - last) / 1000);
  last = now;
  const t = now / 1000;
  ctx.globalCompositeOperation = "source-over";
  ctx.fillStyle = `rgba(14, 15, 19, ${1 - Math.pow(1 - 0.1, dt * 60)})`; // trail fade
  ctx.fillRect(0, 0, W, H);
  ctx.globalCompositeOperation = "lighter";
  for (const p of ps) {
    const [vx, vy] = curl(p.x, p.y, t);
    p.x += vx * 0.03 * dt;
    p.y += vy * 0.03 * dt;
    p.age += dt;
    if (p.age > p.life || p.x < 0 || p.x > W / H || p.y < 0 || p.y > 1) spawn(p);
    const k = p.age / p.life;
    const fade = Math.min(1, k / 0.1) * Math.min(1, (1 - k) / 0.3);
    ctx.fillStyle = `rgba(233, 230, 223, ${0.6 * fade})`;
    ctx.beginPath();
    ctx.arc(p.x * H, p.y * H, p.size, 0, Math.PI * 2);
    ctx.fill();
  }
  requestAnimationFrame(frame);
}
requestAnimationFrame(frame);
```

For depth, scale size, speed and opacity together by a per-particle depth z in 0..1: near
particles are bigger, faster and brighter. Blur the far ones by drawing them larger and fainter.

## Don't

- Don't give all particles the same size or opacity.
- Don't move particles along the gradient of noise; they collect into lines. Use the curl.
- Don't stack additive particles until large areas clip to white.

## Check

- After 30 s, particles are still spread over the frame, not collected in streaks.
- At 30 and 60 fps, trails have the same length.

## Next

- [Raymarch](raymarch.md)
- [Organic flow](../look/organic-flow.md)
