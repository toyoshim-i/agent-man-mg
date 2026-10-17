---
id: type-pairings
title: Type pairings
summary: Use when choosing typefaces. Free fonts (Google Fonts) by mood, with display and support pairings, weights and tracking.
category: table
aliases: [fonts, typefaces, font-pairing]
---
# Type pairings

All families are free on Google Fonts. Load with
`https://fonts.googleapis.com/css2?family=<Name>:wght@<weights>&display=swap`.
Use one family when you can; pair only when the support text needs a different voice.

| Mood | Display | Weights | Support | Notes |
|---|---|---|---|---|
| precise, tech, Swiss | Inter Tight | 600, 800 | Inter 400 | tracking -3% at display sizes |
| neutral, modern | Space Grotesk | 500, 700 | Space Grotesk 400 | slightly quirky, good for tech brands |
| technical, data | IBM Plex Mono | 500 | IBM Plex Sans 400 | labels, counters, HUDs |
| bold, energetic | Archivo | 800, 900, width axis `wdth` 125 | Archivo 400 | wide and heavy for impact |
| condensed impact | Anton | 400 | Inter 500 | uppercase, tracking +2% |
| poster, display | Bebas Neue | 400 | Inter 400 | uppercase only |
| elegant, luxury | Cormorant Garamond | 500, 600 | Inter 300 at large sizes only | tracking +8% uppercase small caps |
| editorial, serif | Instrument Serif | 400, italic | Inter 400 | big italic headlines |
| warm serif | Fraunces | 600, 800 (optical size high) | Fraunces 400 | playful-premium |
| playful, rounded | Nunito | 800, 900 | Nunito 600 | soft, friendly |
| futuristic | Unbounded | 600, 800 | Space Grotesk 400 | very wide, use few words |
| art-school, experimental | Syne | 700, 800 | Space Mono 400 | expressive widths |
| retro computer | VT323 or Press Start 2P | 400 | Space Mono 400 | pixel; only at pixel-exact sizes |
| Japanese | Noto Sans JP | 700, 900 | Noto Sans JP 400 | tracking +2% to +5%; avoid letter stagger on kanji compounds |
| Japanese serif | Noto Serif JP or Shippori Mincho | 600, 800 | Noto Sans JP 400 | elegant, literary |

## Rules

- Weight steps between display and support are 300 or more (800 vs 400).
- Never pair two serifs or two geometric sans. Contrast the voice or use one family.
- On projectors and LED walls, use weight 500 or more everywhere.
- In shaders, render text from an SDF font atlas or pre-rendered texture; do not draw letters
  with hand-made SDF strokes unless the look is deliberately constructed.

## Next

- [Typography](../principle/typography.md)
- [Kinetic type code](../recipe/kinetic-type-code.md)
