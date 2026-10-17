---
id: unreadable-type
title: Unreadable type
summary: Use when text is hard to read - too small, too thin, too fast, low contrast, over busy backgrounds, or moving while being read.
category: rule
aliases: [illegible, text-too-small, text-too-fast, low-contrast-text]
---
# unreadable-type

## Symptom

At 160 px thumbnail the headline is mush. Text leaves before it can be read twice. Thin
letters vanish on the projector. Text sits on bright, detailed texture.

## Cause

Size below 1/25 of frame height; weight below 400; hold shorter than the read time; text
animated during its hold; contrast against the local background below 0.4 L.

## Fix

1. Headline 1/8 to 1/4 of frame height; support at least 1/20.
2. Weight 500 or more on projection, 400 or more on screens.
3. Hold at least `max(1.5, words / 3 + 1.0)` s, still.
4. Put a contrast plate behind text on busy areas: darken by 0.3 L in a soft rectangle.
5. Cut the copy: at most 6 words per card.

## Next

- [Typography](../principle/typography.md)
- [Timing](../principle/timing.md)
