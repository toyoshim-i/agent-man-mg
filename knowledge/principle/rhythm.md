---
id: rhythm
title: Rhythm and music sync
summary: Use when visuals follow music or a beat. Beat math, which events land on which beat, phrase structure, and accents versus constant motion.
category: principle
aliases: [beat, bpm, sync, music, tempo, phrasing]
---
# Rhythm and music sync

## Rule

Sync structure, not every sound. Big changes land on phrase boundaries, hits land on beats,
and continuous motion runs at a fraction of the tempo. If everything pulses on every kick,
nothing is an accent.

## Defaults

| Musical unit | Typical length | Visual event |
|---|---|---|
| beat | 60 / BPM s (0.469 s at 128) | small pulse, light accent, cut on fast sections |
| bar | 4 beats | position change, color swap, camera step |
| phrase | 8 or 16 bars (15 or 30 s at 128) | new scene, new look, new palette |
| drop / section change | after a 4 to 16 bar build | the peak; breath of 1 to 2 beats before it |

| Element | Rate |
|---|---|
| ambient drift | one cycle per 4 to 16 bars |
| pulses on the kick | at most 1 element class; amplitude 3 to 8% of its size |
| hats / high-frequency detail | small things only: sparkle, grain, line jitter |
| cuts | on beats; every 1, 2 or 4 bars; faster only in a peak |

Syncopation: put some accents on the off-beat (beat + 0.5) to avoid a march. One off-beat
accent per bar is plenty.

## How

Beat phase from time and BPM:

```glsl
const float BPM = 124.0;

float beatPhase(float t) { return fract(t * BPM / 60.0); }       // 0..1 within a beat
float barIndex(float t)  { return floor(t * BPM / 240.0); }      // integer bar count
// A pulse that peaks on the beat and decays: sharp attack, exponential release.
float beatPulse(float t, float decay) { return exp(-decay * beatPhase(t)); }
```

`beatPulse(t, 6.0)` gives a kick-like envelope. Use 3.0 for softer, 12.0 for tight.
With live audio, drive the same envelope from the detected kick: see
[Audio reactive](../recipe/audio-reactive.md) and [Sync](../vj/vj-sync.md).

## Don't

- Don't map raw volume to scale. Use an envelope with a fast attack and slow release.
- Don't change scenes mid-phrase unless it is a deliberate break.
- Don't pulse the whole frame brightness on every kick:
  [strobe-risk](../rule/strobe-risk.md) and [monotony](../rule/monotony.md).

## Check

- Major event times are multiples of the beat length.
- Only one element class pulses on the kick.

## Next

- [Energy](energy.md)
- [BPM table](../table/bpm.md)
