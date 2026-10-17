---
id: brief
title: Step 1 - Brief
summary: Use when a request arrives. Turns it into a spec with format, duration, medium, audience, purpose and three mood words.
category: process
aliases: [spec, requirements, step-1]
---
# Step 1: Brief

## Rule

Before any design, write the spec block below. Every field gets a value. If the request
does not say, choose the default and mark it `(assumed)`. Do not ask the user unless a
wrong guess would waste the whole piece (wrong medium, wrong text).

## The spec block

```text
purpose:     what the viewer should feel or know afterwards, one line
audience:    who watches, where (club, phone feed, conference screen, website hero)
medium:      GLSL fragment shader | HTML/CSS/JS | Canvas 2D | WebGL | After Effects | other
format:      width x height, aspect, fps
duration:    seconds, or "loop N s" for loops, or "live" for VJ
content:     exact text, logo, assets that must appear
music:       BPM and genre if any, or "none"
mood words:  three adjectives, ranked (first wins conflicts)
must avoid:  anything the brief forbids or the audience would hate
done when:   one observable acceptance test
```

## Defaults

| Field | Default when unspecified |
|---|---|
| medium | GLSL fragment shader, Shadertoy conventions (`mainImage`, `iTime`, `iResolution`) |
| format | 1920x1080, 16:9, 60 fps for live and shader, 30 fps for video |
| social formats | 1080x1920 (9:16) for stories and reels, 1080x1350 (4:5) for feeds |
| duration | logo sting 3 to 6 s; title card 4 to 8 s; social clip 10 to 15 s; VJ loop 8 or 16 bars |
| music | none; if VJ, assume 124 BPM four-on-the-floor |
| mood words | derive from the subject: a bank is "trustworthy, calm, precise" |

## How to pick mood words

1. Name the subject and the audience in one line.
2. Write five adjectives a fan of the subject would use. Not "cool", "nice", "modern":
   those carry no numbers.
3. Keep three. Rank them. The first one decides every conflict later.
4. Look each up in the [Mood map](../table/mood-map.md) now, to check it has parameters.
   If a word is not in the table, replace it with the nearest one that is.

## Don't

- Don't start writing code in this step.
- Don't keep a mood word that contradicts another ("calm" with "aggressive"). Pick one.
- Don't leave `done when` vague. "Looks good" is not a test; "the logo is readable at
  t = 3.0 s in a 320 px wide thumbnail" is.

## Check

- Every field has a value or `(assumed)`.
- The three mood words all exist in the mood map.
- Duration is a whole number of beats when there is music: see [BPM table](../table/bpm.md).

## Next

- [Concept](2-concept.md)
- Live visuals: [VJ set](../vj/vj-set.md)
