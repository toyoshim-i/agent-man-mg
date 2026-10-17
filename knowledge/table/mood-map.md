---
id: mood-map
title: Mood map
summary: Use when turning mood words into parameters. Each mood gives speed, easing, palette, density, shapes, texture, camera and type as numbers.
category: table
aliases: [mood, moods, feel, adjectives, taste]
---
# Mood map

Find each of the three mood words from the brief. The first word's row is the base; the
second and third change only the columns where they are strong. If a word is missing, use the
nearest row and write the substitution down.

## Motion

| Mood | Speed factor | Ease family | Drift cycles/s | Stagger s | Overshoot | Camera |
|---|---|---|---|---|---|---|
| calm, serene | 1.5 | inOutSine | 0.02 to 0.05 | 0.08 | no | static or push 3% |
| elegant, luxury | 1.3 | outExpo, long | 0.03 to 0.06 | 0.06 | no | slow push 4% |
| precise, tech, trustworthy | 1.0 | outExpo, inOutCubic | 0.05 | 0.04 | no | static, cuts |
| playful, friendly | 0.9 | outBack, spring 0.6 | 0.08 | 0.05 | yes, 10% | gentle bounce |
| energetic, bold | 0.7 | outQuint | 0.1 to 0.2 | 0.03 | small, 5% | cuts on beats, shake on hits |
| aggressive, raw | 0.6 | outQuint, hard cuts | 0.15 to 0.3 | 0.02 | no | shake, roll 2 degrees |
| dark, tense | 1.2 | inOutCubic, sudden cuts | 0.02 to 0.04 | 0.08 | no | slow push 6%, rare jolt |
| dreamy, ethereal | 1.6 | inOutSine | 0.02 to 0.04 | 0.1 | no | drift, defocus |
| nostalgic, retro | 1.1 | outCubic | 0.04 | 0.05 | no | static, slight jitter |
| hypnotic, deep | 1.4 | inOutSine, periodic | 0.03 per bar | 0.06 | no | slow orbit |
| euphoric, uplifting | 0.9 | outExpo | 0.1 | 0.04 | small | pull-outs, rising |
| organic, natural | 1.3 | spring 0.8, inOutSine | 0.03 | jittered 0.06 | soft | handheld drift 0.3% |

## Image

| Mood | Value key | Chroma | Hues | Density | Shapes, edges | Texture |
|---|---|---|---|---|---|---|
| calm, serene | dark or light, low contrast 0.5 | 0.02 to 0.08 | analogous, cool | 0.15 | circles, soft edges | grain 0.02 |
| elegant, luxury | dark, high contrast 0.75 | 0.01 to 0.06, one gold or deep accent | 1 hue + neutrals | 0.15 | thin lines, serif type | grain 0.03, vignette 0.25 |
| precise, tech, trustworthy | light or dark, contrast 0.7 | 0.02 to 0.12 | blue to cyan + 1 accent | 0.3 | grid, rects, 1 px lines | grain 0, dither only |
| playful, friendly | light, contrast 0.5 | 0.12 to 0.2 | 3 hues, warm | 0.4 | rounded, blobs | grain 0.03, paper |
| energetic, bold | dark, contrast 0.8 | accent 0.2 | complementary pair | 0.6 | big type, hard edges | grain 0.04 |
| aggressive, raw | dark, contrast 0.9 | 0 to 0.2, red or acid accent | 1 accent + mono | 0.7 | jagged, cropped type | grain 0.06, glitch bursts |
| dark, tense | dark, mean L 0.1 | 0.02 to 0.06 | desaturated cool, one red | 0.2 | slivers, slits of light | grain 0.05, vignette 0.35 |
| dreamy, ethereal | light-mid, low contrast 0.4 | 0.04 to 0.1 | pastel analogous | 0.3 | blurred blobs, haze | bloom 0.2, grain 0.02 |
| nostalgic, retro | mid, lifted blacks L 0.05 | 0.06 to 0.12, warm cast | warm + faded teal | 0.4 | era-specific | grain 0.05, CRT or film |
| hypnotic, deep | dark, mean L 0.12 | 0 to 0.08 | 1 hue | 0.25 | rings, lines, repetition | grain 0.03, feedback trails |
| euphoric, uplifting | light-mid, bright peaks | 0.1 to 0.2 | warm to magenta | 0.5 | rays, open space, flares | bloom 0.15 |
| organic, natural | mid, contrast 0.5 | 0.04 to 0.1, earth | greens, ochres | 0.35 | noise forms, curves | grain 0.04, paper |

Value key "contrast" is the OKLCH L difference between background and main.

## Next

- [Design sheet](../process/3-design.md)
- [Palettes](palettes.md)
- Looks that bundle these: [Swiss](../look/swiss.md), [Organic flow](../look/organic-flow.md),
  [Neon grid](../look/neon-grid.md)
