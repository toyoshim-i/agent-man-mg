---
id: vj-projection
title: Projection and LED output
summary: Use when visuals go to a projector, LED wall or venue screen. What survives - contrast, line width, black levels, brightness, resolution and mapping margins.
category: vj
aliases: [projection, projector, led-wall, venue, output, mapping]
---
# Projection and LED output

## Rule

The venue changes the image. Projectors in a lit, hazy room lose shadows and fine detail;
LED walls are coarse, bright and saturated. Design for the worst case: bold shapes, thick
lines, black as empty space, and test on the real output before the show.

## Defaults

| Item | Projector | LED wall |
|---|---|---|
| empty space | pure black (projector off) | pure black (LEDs off), saves power and heat |
| darkest detail that survives | L 0.2 and above | L 0.1 and above |
| thinnest line | 3 px at 1080p, 6 px preferred | 1 LED module pixel minimum, 2 to 3 preferred |
| smallest text | 1/18 of frame height, weight 500 or more | 1/12 of panel height |
| mean brightness | below L 0.4 for comfort over a set | 60 to 70% of the wall's maximum |
| mid-gray large fields | avoid: lights up the room and haze | avoid: glare |
| edge margin | 5% for keystone and mapping slop | none, but check panel seams |
| refresh issues | none | phone cameras show rolling bands at some rates; avoid 1 px horizontal stripes |

## Haze

Haze makes beams visible but lowers contrast on the screen. In heavy haze, raise contrast
(main L 0.9 on black), simplify shapes, and avoid soft gradients.

## How

1. Build the preview with blacks at 0 and check the frame darkened by 30%: does the subject
   survive?
2. Lift the frame by 0.05 L (the room's ambient light) and check again.
3. Downsample to the LED wall's real resolution (often 1/4 to 1/8 of 1080p) and look.
4. At the venue, show a test pattern: 1 to 8 px lines, a gray ramp, the palette swatches.

## Don't

- Don't rely on shadow detail or dark gradients: [washed-out](../rule/washed-out.md).
- Don't use thin serifs or light weights.
- Don't output large saturated red areas on LED walls for long periods; they glare.

## Next

- [Flash safety](vj-safety.md)
- [Performance](vj-performance.md)
