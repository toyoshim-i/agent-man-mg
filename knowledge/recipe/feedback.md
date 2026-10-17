---
id: feedback
title: Feedback and trails
summary: Use when you want trails, echoes, smears or infinite-zoom tunnels. A self-reading buffer with decay, transform and frame-rate independence.
category: recipe
aliases: [trails, echo, buffer, ping-pong, persistence, video-feedback]
---
# Feedback and trails

## Rule

Feedback is a buffer that reads its own previous frame, transforms it slightly, fades it, and
draws new content on top. Small transforms per frame become large structures over time.
Decay and transform amounts must be scaled by frame time, or the look changes with frame rate.

## Defaults

| Parameter | Per frame at 60 fps |
|---|---|
| decay (keep factor) | 0.90 to 0.98 (trails); 0.985 to 0.995 (long echoes) |
| sampling scale | 0.995 to 1.01 (below 1 expands outward, above 1 pulls into a tunnel) |
| rotation | 0.001 to 0.005 rad |
| offset drift | 0 to 0.001 uv |
| hue or tint shift on decay | very small: multiply by a color 0.99 to 1.0 per channel |
| new content opacity | 0.3 to 1.0 |

## How

This is Buffer A in Shadertoy, with iChannel0 set to Buffer A itself (linear filtering).
The Image pass reads Buffer A and applies the finish.

```glsl
mat2 rot(float a) { float c = cos(a); float s = sin(a); return mat2(c, s, -s, c); }

void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    vec2 res = iResolution.xy;
    vec2 uv = (2.0 * fragCoord - res) / res.y;
    float dt60 = iTimeDelta * 60.0;                      // 1.0 at 60 fps

    // Previous frame, transformed around the center.
    vec2 p = rot(0.003 * dt60) * uv * pow(0.996, dt60);
    vec2 st = (p * res.y + res) / (2.0 * res);
    vec3 prev = texture(iChannel0, st).rgb;
    prev *= pow(vec3(0.965, 0.955, 0.94), vec3(dt60));   // decay, warm tint on the tail

    // New content: a small orbiting light.
    vec2 c = 0.6 * vec2(cos(iTime * 0.7), sin(iTime * 0.9));
    float d = length(uv - c);
    vec3 fresh = vec3(1.0, 0.45, 0.2) * smoothstep(0.06, 0.0, d);

    vec3 col = max(prev, fresh);                          // max keeps trails from blowing out
    if (iFrame == 0) col = vec3(0.0);
    fragColor = vec4(col, 1.0);
}
```

Combine with `max` for clean trails, `+` with lower opacity for glowing build-up (watch for
white-out), `mix` for soft ghosting.

## Don't

- Don't use decay 1.0 or zoom far from 1.0; the frame fills or empties within a second.
- Don't feed back a frame that already has grain and dither; add them in the Image pass only,
  or noise accumulates into mush.
- Don't feed back in 8-bit buffers for long echoes: use float buffers, or the tail bands
  ([banding](../rule/banding.md)).

## Check

- At 30 and at 60 fps, trails have the same length in seconds.
- After 60 s of running, the frame is not saturated white or empty.

## Next

- [Particles](particles.md)
- [Hypnotic loops in VJ sets](../vj/vj-set.md)
