---
id: jitter
title: Audio jitter
summary: Use when audio-reactive values flicker, twitch every frame, react to noise, or pump unevenly with the music.
category: rule
aliases: [flicker, twitchy, audio-flicker, nervous, unstable-reaction]
---
# jitter

## Symptom

Audio-driven elements twitch on every frame, even between beats. The reaction is too big on
loud tracks and absent on quiet ones. Silence still makes things shake.

## Cause

Raw FFT or volume mapped directly to a parameter; no envelope; no auto-gain; linear mapping
of a dB or power value; too large an amount.

## Fix

1. Average bins into bands ([Audio reactive](../recipe/audio-reactive.md)).
2. Normalize with auto-gain (peak decay 5 to 10 s).
3. Smooth with an envelope: attack 5 to 20 ms, release 150 to 400 ms for kicks.
4. Shape with `smoothstep(0.2, 0.9, x)` so the noise floor maps to 0.
5. Reduce amounts: scale 3 to 8%, brightness 5 to 15%.
6. For hits, detect onsets and trigger a choreographed envelope instead.

## Next

- [Rhythm](../principle/rhythm.md)
- [Sync](../vj/vj-sync.md)
