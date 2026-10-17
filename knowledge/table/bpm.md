---
id: bpm
title: BPM table
summary: Use when timing visuals to music. Beat, bar and phrase lengths in seconds and frames for common tempos and genres.
category: table
aliases: [bpm-table, tempo, beats, bars]
---
# BPM table

Beat = 60 / BPM seconds. Bar = 4 beats. Phrase = 8 or 16 bars.

| BPM | beat ms | bar s | 8 bars s | 16 bars s | frames/beat at 30 | frames/beat at 60 |
|---|---|---|---|---|---|---|
| 70 | 857 | 3.429 | 27.43 | 54.86 | 25.71 | 51.43 |
| 85 | 706 | 2.824 | 22.59 | 45.18 | 21.18 | 42.35 |
| 90 | 667 | 2.667 | 21.33 | 42.67 | 20.00 | 40.00 |
| 100 | 600 | 2.400 | 19.20 | 38.40 | 18.00 | 36.00 |
| 110 | 545 | 2.182 | 17.45 | 34.91 | 16.36 | 32.73 |
| 120 | 500 | 2.000 | 16.00 | 32.00 | 15.00 | 30.00 |
| 124 | 484 | 1.935 | 15.48 | 30.97 | 14.52 | 29.03 |
| 126 | 476 | 1.905 | 15.24 | 30.48 | 14.29 | 28.57 |
| 128 | 469 | 1.875 | 15.00 | 30.00 | 14.06 | 28.12 |
| 130 | 462 | 1.846 | 14.77 | 29.54 | 13.85 | 27.69 |
| 135 | 444 | 1.778 | 14.22 | 28.44 | 13.33 | 26.67 |
| 140 | 429 | 1.714 | 13.71 | 27.43 | 12.86 | 25.71 |
| 150 | 400 | 1.600 | 12.80 | 25.60 | 12.00 | 24.00 |
| 160 | 375 | 1.500 | 12.00 | 24.00 | 11.25 | 22.50 |
| 170 | 353 | 1.412 | 11.29 | 22.59 | 10.59 | 21.18 |
| 174 | 345 | 1.379 | 11.03 | 22.07 | 10.34 | 20.69 |

## Genres

| Genre | BPM | Feel for visuals |
|---|---|---|
| ambient, downtempo | 60 to 100 | phrase-level change only; no beat pulses |
| hip-hop | 80 to 100 | accents on 2 and 4 (snare); swing |
| house, deep house | 118 to 126 | warm, steady; pulse on kick, change every 8 bars |
| techno | 125 to 140 | hypnotic; minimal palette, long builds, change every 16 bars |
| trance | 130 to 140 | long builds to euphoric drops; brightness and openness at the drop |
| dubstep, trap | 140 to 150 (half-time feel 70 to 75) | heavy impacts on the half-time snare |
| drum and bass | 170 to 176 | fast detail, slow big shapes (half-time camera) |

Non-integer frames per beat drift against the frame grid. Compute event times from the
beat in seconds, never by counting frames.

## Next

- [Rhythm](../principle/rhythm.md)
- [Sync](../vj/vj-sync.md)
