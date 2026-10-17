---
id: palettes
title: Palettes
summary: Use when you need a tested palette. Twelve four-color sets with roles, OKLCH values and the mood each fits.
category: table
aliases: [palette-table, colors, colour-sets, swatches]
---
# Palettes

Each set is background, main, second, accent, with OKLCH (L C h). Every set passes the
[Color](../principle/color.md) contrast rules. Use the accent on at most 10% of the frame.
In shaders, convert hex to linear with `hexToLinear` from
[Palette code](../recipe/palette-code.md).

| Name | Background | Main | Second | Accent | Fits |
|---|---|---|---|---|---|
| ink-night | #0E0F13 (0.17 0.01 274) | #E9E6DF (0.93 0.01 87) | #6B7280 (0.55 0.02 264) | #FF5A36 (0.68 0.21 34) | precise, premium, default dark |
| paper | #F3F0EA (0.96 0.01 85) | #16161A (0.20 0.01 286) | #8A877F (0.62 0.01 90) | #E63B2E (0.61 0.21 29) | editorial, default light |
| swiss-red | #F2F0EB (0.96 0.01 89) | #111111 (0.18 0.00 90) | #BDBAB3 (0.79 0.01 87) | #E1251B (0.58 0.22 29) | Swiss, bold, typographic |
| deep-sea | #07131A (0.18 0.02 235) | #8FD3E0 (0.83 0.07 211) | #1E5A6B (0.44 0.07 220) | #F2C66D (0.85 0.12 83) | calm, hypnotic, ambient |
| ember | #120B0A (0.16 0.01 28) | #F4E3D0 (0.92 0.03 71) | #8C3B22 (0.46 0.12 38) | #FF8A3D (0.75 0.17 51) | warm, elegant, intimate |
| acid-club | #0A0A0A (0.14 0.00 90) | #EDEDED (0.95 0.00 90) | #3A3A3A (0.35 0.00 90) | #C6FF3D (0.93 0.22 125) | techno, aggressive, raw |
| synth-dusk | #120A24 (0.17 0.05 295) | #F7E8FF (0.95 0.03 314) | #5B2A86 (0.40 0.15 305) | #FF3D8B (0.67 0.23 2) | retro-future, euphoric |
| forest-mist | #0F1A14 (0.20 0.02 160) | #D7E3D2 (0.90 0.03 136) | #4E6B57 (0.50 0.05 154) | #E8B04B (0.79 0.13 79) | organic, natural, calm |
| chrome-cold | #0B0D10 (0.16 0.01 258) | #C9D3DD (0.86 0.02 248) | #5C6B7A (0.52 0.03 248) | #7AF0FF (0.89 0.11 206) | tech, futuristic, clinical |
| riso | #F5EFE3 (0.95 0.02 85) | #2B59C3 (0.50 0.17 264) | #FF6F91 (0.73 0.18 8) | #FFD23F (0.88 0.16 91) | playful, print, friendly |
| mono-op | #0B0B0B (0.15 0.00 90) | #F5F5F5 (0.97 0.00 90) | #7A7A7A (0.58 0.00 90) | #FF3B30 (0.65 0.23 29) | op-art, minimal, hypnotic |
| trust | #0B1B2B (0.22 0.04 250) | #F4F7FA (0.97 0.01 248) | #2F6FB2 (0.53 0.12 252) | #3DDC97 (0.80 0.16 160) | corporate, fintech, health |

## Making a variant

- Change the hue of the whole set by the same angle; keep L and C. Contrast is preserved.
- Swap dark and light: exchange background and main, keep second and accent, then check the
  accent's L is still 0.25 or more from the new background.
- For glow on dark sets, use the accent and the main as emission colors, in linear space.

## Next

- [Color](../principle/color.md)
- [Palette code](../recipe/palette-code.md)
