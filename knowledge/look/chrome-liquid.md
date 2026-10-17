---
id: chrome-liquid
title: Chrome and liquid metal
summary: Use when the brief is futuristic, luxury tech, Y2K, fashion or product launch. Reflective blobs and type, environment-mapped highlights, slow morphing.
category: look
aliases: [chrome, liquid-metal, y2k, metallic, reflective, iridescent]
---
# Chrome and liquid metal

Surfaces that are only reflections. Chrome reads through sharp contrast between dark and
bright bands of an imagined environment, and through slow, heavy morphing.

## Design sheet

```text
look:      chrome-liquid
palette:   chrome-cold, or a mono base with one iridescent band (thin-film hue shift)
type:      Unbounded 800 or Syne 800, chrome-filled, large, few words
shapes:    raymarched blobs with smooth union k 0.3 to 0.5; extruded type
motion:    inOutCubic; slow morphs 3 to 6 s; spring 0.7 on merges
speed:     factor 1.2
density:   0.3
texture:   grain 0.02, vignette 0.2, bloom 0.1 on specular highlights
camera:    slow orbit 15 to 25 degrees per shot
```

## Signature moves

- Fake environment: reflect the ray into a procedural sky of 2 to 4 soft horizontal bands
  plus one hard bright window; that is what makes it look like chrome.
- Fresnel: reflections stronger at grazing angles, `pow(1 - dot(n, -rd), 5)`.
- Thin-film iridescence: hue shift by view angle in a narrow range (60 degrees of hue).
- Blobs merge and split with smooth union, timed to phrases.

## Don't

- No uniform gray environment; chrome needs high contrast to reflect.
- No fast motion; liquid metal is heavy ([floaty](../rule/floaty.md) if it drifts without
  weight).
- No rainbow iridescence over the whole surface.

## Build with

- [Raymarch](../recipe/raymarch.md) (replace diffuse shading with reflection lookup).
- [Post chain](../recipe/post-chain.md) for highlight bloom.

## Next

- [Ink and paper](ink-paper.md)
