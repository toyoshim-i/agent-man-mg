---
id: swiss
title: Swiss / International
summary: Use when the brief is precise, corporate, editorial, architectural or typographic. Grid, flat color, big sans type, hard cuts and expo easing.
category: look
aliases: [international-style, minimal, grid-design, corporate, editorial]
---
# Swiss / International

The design-school default. Order, grid, flat color, one strong typeface, motion as
precise as a ruler. It works for almost any brand that wants to look confident.

## Design sheet

```text
look:      swiss
palette:   swiss-red or paper (light), ink-night (dark)
type:      Inter Tight 800 display, Inter 400 support; headline 1/6 of height; tracking -3%
shapes:    rectangles, lines, circles; stroke 0.3% of height; corner radius 0
motion:    outExpo entrances, inExpo exits, inOutCubic moves; no overshoot
speed:     factor 1.0; drift 0
density:   0.3
texture:   grain 0, vignette 0, dither only
camera:    static; scene changes by hard cut or grid-aligned wipe
```

## Signature moves

- Elements slide along grid columns and stop exactly on grid lines.
- Color blocks wipe in from one edge, revealing type behind them (mask rise).
- A single red element (the accent) is the only thing that moves in a given beat.
- Asymmetric layouts: type flush left on column 2, big negative space on the right.

## Don't

- No gradients, glows, grain or blur.
- No rotation other than 90-degree steps.
- No centered text blocks longer than one line.

## Build with

- [Composition](../principle/composition.md) for the 12-column grid.
- [Kinetic type code](../recipe/kinetic-type-code.md) for the type.
- [SDF 2D](../recipe/sdf-2d.md) for shapes in shaders, with corner radius 0.

## Next

- [Brutal type](brutal-type.md)
- [Design sheet](../process/3-design.md)
