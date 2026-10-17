---
id: transitions
title: Transitions
summary: Use when moving from one scene, state or clip to the next. Which transition for which reason, durations, and how to make it motivated.
category: principle
aliases: [cut, wipe, match-cut, dissolve, scene-change]
---
# Transitions

## Rule

A transition needs a reason, and the best reason is motion that already exists: an element's
movement or shape carries the eye into the next scene. Prefer cuts and match cuts; use
dissolves only for a change of time or mood. Use one transition vocabulary per piece.

## Defaults

| Transition | When | Duration |
|---|---|---|
| hard cut on the beat | default between scenes with music | 0 |
| match cut | shape, position or motion direction repeats across scenes | 0 |
| motivated wipe | an element sweeps across and reveals the next scene behind it | 0.4 to 0.7 s, inOutCubic |
| mask grow | a shape from scene A scales to fill the frame and becomes scene B | 0.5 to 0.9 s, inOutExpo |
| whip / smear | fast camera move with motion blur bridging the cut | 0.2 to 0.33 s (6 to 10 frames at 30) |
| cross dissolve | time passes, mood shifts, ambient VJ blends | 0.3 to 0.5 s video; 1 to 4 bars live |
| luma fade through black | end of a section, a breath | 0.2 to 0.4 s out, hold 2 to 6 frames, 0.3 s in |

## How

1. For each scene change in the timing chart, write what carries the eye: an object, a
   direction, a shape, a color.
2. If something carries it, use a match cut or a motivated wipe built from that thing.
3. If nothing does, cut on the beat. Hard cuts are not a failure; random effects are.
4. Keep direction: an exit to the right continues as an entrance from the left, at the same
   speed.

A shader transition between two scenes A and B, driven by a mask:

```glsl
// p: 0..1 progress, already eased. Returns the blend weight of scene B.
float wipeMask(vec2 uv, float p, vec2 dir, float softness) {
    float d = dot(uv, normalize(dir));          // position along the wipe direction
    float edge = mix(-1.2, 1.2, p);             // wipe front sweeps across the frame
    return smoothstep(edge + softness, edge - softness, d);
}
// col = mix(sceneA, sceneB, wipeMask(uv, inOutCubic(p), vec2(1.0, 0.2), 0.02));
```

## Don't

- Don't use a different transition for every cut.
- Don't use slow dissolves between energetic scenes. They kill the beat.
- Don't do a 3D cube spin, page curl or star wipe unless the brief is a parody.

## Check

- Every transition in the chart has a written reason.
- Motion direction is continuous across each transition.

## Next

- [Camera](camera.md)
- [VJ layers](../vj/vj-layers.md)
