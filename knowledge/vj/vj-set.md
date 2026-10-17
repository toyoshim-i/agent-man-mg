---
id: vj-set
title: VJ set structure
summary: Use when planning live visuals for a set or a night. Energy arc over hours, clip library by energy level, when to change, and what to save for the peak.
category: vj
aliases: [set, live-set, vj, performance-arc, clip-library, live-visuals]
---
# VJ set structure

## Rule

Follow the music's energy, one step behind and one step quieter, and save the strongest
material for the peak. A VJ set is a long piece with a curve, not a playlist of loops: the
[Energy](../principle/energy.md) rules apply at the scale of an hour.

## The arc

| Phase of the night | Visual energy | Material |
|---|---|---|
| doors, warm-up | 1 to 3 | slow ambient loops, low brightness (mean L below 0.15), mono or one hue |
| build | 3 to 6 | geometric loops on the beat, second hue enters, more density |
| peak hour | 6 to 9 | strongest pieces, accents on drops, full palette, fastest cuts |
| breakdowns inside the peak | drop to 2 to 4 | strip to one element; darkness before the drop |
| closing | 4 to 2 | return to warm-up material, slower, the night's motif one last time |

## Clip library

Tag every loop or scene with:

```text
energy:  1..5
hue:     main hue name (e.g. cyan, ember, mono)
tempo:   loop length in bars, and whether it is beat-synced
role:    base (full-frame background) | mid (rhythmic shapes) | top (accents, text)
```

Keep at least 3 base loops per energy level. Changing between two loops of the same energy
and hue is invisible; changing energy or hue is an event, so do it on a phrase.

## When to change

- Hold a scene for 1 to 4 phrases (30 s to 2 min at 128 BPM). Faster changing is tiring.
- Change on the first beat of a phrase. Before a drop, strip down for the last 1 to 2 bars,
  then hit the new scene on the drop.
- Blackout (or near-black) for 1 to 2 beats before the biggest drop of the night. Once.

## Don't

- Don't open with your best scene.
- Don't run peak energy through warm-up music; it fights the DJ.
- Don't mix two busy layers: see [VJ layers](vj-layers.md).

## Next

- [Sync](vj-sync.md)
- [Flash safety](vj-safety.md)
- [Loops](../principle/loops.md)
