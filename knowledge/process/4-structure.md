---
id: structure
title: Step 4 - Structure
summary: Use when the design sheet is done. Plans time - sections, energy curve and a timing chart in seconds - before building.
category: process
aliases: [timing-chart, dope-sheet, storyboard, step-4]
---
# Step 4: Structure

## Rule

Time needs a shape. Write an energy curve (0 to 10 per section) and a timing chart that lists
every event with start time, duration and easing. Code later reads this chart line by line.
A piece with a flat energy curve fails, however good each frame is: see
[Monotony](../rule/monotony.md).

## Sections

Most pieces use four sections. Durations are fractions of the whole.

| Section | Share | Energy | What happens |
|---|---|---|---|
| Setup | 0.15 to 0.25 | 2 to 4 | space and material established, one element moves |
| Build | 0.30 to 0.40 | 4 to 7 | elements gather, motion accelerates, density rises |
| Peak | 0.15 to 0.25 | 8 to 10 | the concept's verb completes; logo or text resolves |
| Release | 0.15 to 0.25 | 3 to 5 | hold for reading, then a gentle settle or exit |

A loop replaces Release with a return to the Setup state. See [Loops](../principle/loops.md).
Before the peak, put a breath: 2 to 8 frames where motion stops or the frame darkens.
See [Energy](../principle/energy.md).

## Timing chart format

```text
t_start  dur    element      property      from -> to      ease
0.00     0.60   ring         scale         0.0 -> 1.0      outExpo
0.15     0.45   ring         opacity       0 -> 1          outCubic
0.90     0.08   all          (breath)      hold            -
0.98     0.50   logo         reveal mask   0 -> 1          inOutCubic
1.48     1.50   logo         (hold)        read time       -
```

Times in seconds. With music, every `t_start` of a major event lands on a beat: compute with
the [BPM table](../table/bpm.md). Then turn the chart into code with
[Timeline](../recipe/timeline.md).

## How

1. Split the duration into sections using the table. Round to beats if there is music.
2. Write the energy number for each section. The highest must be the peak, and the one right
   before it must be lower than the build (the breath).
3. List events. Each section gets one primary event; secondary events start 2 to 6 frames
   after their primary. See [Overlap](../principle/overlap.md).
4. Add holds. Any text gets its read time from [Timing](../principle/timing.md).
5. Check that no two primary events overlap. Two stars at once means none.

## Don't

- Don't start every element at t = 0.
- Don't end on motion. End on a hold of at least 0.5 s, or on the loop seam.
- Don't plan a constant-speed loop with no accents. Even ambient loops need one event per
  4 to 8 bars.

## Check

- Energy curve has one maximum and a dip right before it.
- Every text element has a hold at least as long as its read time.
- With music, major `t_start` values divide evenly by the beat length.

## Next

- [Build](5-build.md)
