---
id: typography
title: Typography in motion
summary: Use when text appears on screen. Sizes, tracking, line length, how to reveal and exit text, and what never to animate.
category: principle
aliases: [type, text, kinetic-typography, fonts, lettering]
---
# Typography in motion

## Rule

Text in motion must still be read. Big, few words, high contrast, held still while read.
Animate the reveal and the exit; keep the text still in between. One typeface family is
best, two is the maximum: one display, one support.

## Defaults

| Parameter | Value (at 1080 px frame height) |
|---|---|
| headline size | 1/8 to 1/4 of frame height (135 to 270 px) |
| support text size | 1/20 to 1/14 of frame height (54 to 77 px) |
| smallest readable | 1/25 of frame height (43 px); on projection 1/18 (60 px) |
| words per line | 2 to 6 |
| lines on screen | at most 3 |
| tracking, uppercase | +5 to +10% (0.05em to 0.1em) |
| tracking, large display lowercase | -2 to -4% |
| line height, display | 0.95 to 1.1 |
| weight contrast | one step apart at least (400 vs 700), never 400 vs 500 |
| letter stagger on reveal | 0.02 to 0.04 s per letter; 0.06 to 0.1 s per word |
| reveal travel | 0.3 to 0.6 em, with outExpo, opacity 0 to 1 over the first 60% |
| read hold | `max(1.5, words / 3 + 1.0)` s, from [Timing](timing.md) |

## Reveal techniques, in order of default preference

1. **Mask rise**: each line slides up from behind a mask at its baseline. Clean, premium.
2. **Word stagger**: words fade and rise 0.4 em, staggered by word.
3. **Letter stagger**: for short words (at most 8 letters) and display moments only.
4. **Tracking settle**: tracking from +30% to the final value with outExpo, plus fade. Elegant.
5. **Scale punch**: from 115% to 100% with outQuint in 0.25 s. Aggressive, on a beat.

Exits use the same technique reversed, 20 to 30% faster, with inExpo.

## How

1. Write the exact text. Cut it until each card has at most 6 words.
2. Choose type from [Type pairings](../table/type-pairings.md) by mood.
3. Set sizes from the table. Check the thumbnail test at 160 px wide.
4. Implement with [Kinetic type code](../recipe/kinetic-type-code.md).

## Don't

- Don't rotate, skew or wobble text while it is being read.
- Don't put text over busy texture without a contrast plate (darken behind it by 0.3 L).
- Don't letter-stagger long sentences. It doubles the read time.
- Don't use thin weights (below 400) under 1/14 of frame height, or on projectors.

## Check

- Thumbnail at 160 px: headline readable.
- Value contrast of text vs what is directly behind it: at least 0.4 L.
- Fails: [unreadable-type](../rule/unreadable-type.md).

## Next

- [Rhythm](rhythm.md)
- [Kinetic quote example](../example/example-kinetic-quote.md)
