---
id: vj-sync
title: Sync to music live
summary: Use when locking live visuals to a DJ or band. Tempo sources (Link, MIDI clock, tap), phase and downbeat alignment, latency compensation and quantized triggers.
category: vj
aliases: [sync, tempo-sync, ableton-link, midi-clock, tap-tempo, latency, quantize]
---
# Sync to music live

## Rule

Tempo comes from the music source when possible, then from tap tempo, and audio onsets only
nudge the phase. Visual events are quantized to the next beat or bar. Shift everything
earlier by the measured output latency, or every hit lands late.

## Tempo sources, best first

| Source | Precision | Notes |
|---|---|---|
| Ableton Link | tempo and phase, network | standard with DJ software and Ableton; gives beat and bar phase |
| MIDI clock | 24 pulses per quarter note | tempo is solid; downbeat needs Song Position or a manual reset |
| OSC from the DJ software | varies | often only BPM |
| tap tempo | about 1 BPM after 8 taps | average the last 4 to 8 intervals; re-tap the downbeat to fix phase |
| audio beat tracking | drifts in breakdowns | use for phase correction only, never as the only clock |

## Phase and latency

```text
beat  = (now - downbeatTime) * BPM / 60      // continuous beat count
phase = fract(beat)                          // 0..1 within the beat
bar   = floor(beat / 4)
```

- Re-anchor `downbeatTime` with a button press on a phrase's first beat.
- Output latency (GPU, scaler, projector) is 30 to 120 ms. Measure: film a beat flash on
  the screen with the speaker in shot, count frames. Then render at `now + latency`.
- Audio analysis latency (FFT window) is 20 to 50 ms more; envelopes from
  [Audio reactive](../recipe/audio-reactive.md) also lag by their attack.

## Quantized triggers

When the operator presses a trigger, schedule it for the next beat, half bar or bar instead of
firing it now. Default quantize: 1 beat for accents, 1 bar for scene changes, 4 bars for palette
changes.

## Don't

- Don't drive scene timing from audio onsets alone; breakdowns have no kicks.
- Don't animate at a free-running speed next to beat-synced layers; the mismatch reads as drift.
- Don't forget latency; 100 ms late is visibly late on a kick.

## Next

- [Rhythm](../principle/rhythm.md)
- [BPM table](../table/bpm.md)
- [Live controls](vj-controls.md)
