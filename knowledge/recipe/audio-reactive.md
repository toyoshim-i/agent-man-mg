---
id: audio-reactive
title: Audio reactive
summary: Use when visuals respond to sound. Frequency bands, envelope followers with attack and release, auto-gain, kick onset detection, and mapping amounts.
category: recipe
aliases: [audio, fft, music-reactive, envelope, onset, kick-detection, spectrum]
---
# Audio reactive

## Rule

Never map raw FFT values to visuals. Raw audio flickers every frame. Pipeline:
**band energy, then auto-gain, then an envelope (fast attack, slow release), then a curve,
then a small amount.** Detect onsets (kicks, snares) as events, and let events trigger
choreographed motion rather than wobble.

## Defaults

| Band | Hz | Drives |
|---|---|---|
| sub + kick | 30 to 120 | scale pulse, camera nudge, light flash on the star only |
| bass body | 120 to 300 | slow color warmth, density |
| mids | 300 to 2000 | shape deformation, line thickness |
| presence, snare crack | 2000 to 5000 | cuts, glitch bursts, accents |
| air, hats | 6000 to 14000 | sparkle, grain amount, small particle jitter |

| Stage | Value |
|---|---|
| envelope attack | 5 to 20 ms (kick), 30 to 80 ms (pads) |
| envelope release | 150 to 400 ms (kick), 500 to 1500 ms (pads) |
| auto-gain peak decay | 5 to 10 s |
| onset threshold | flux above mean + 1.5 standard deviations |
| onset cooldown | 0.1 to 0.15 s |
| mapping curve | `smoothstep(0.2, 0.9, x)` or `x * x` |
| amounts | scale 3 to 8%, brightness 5 to 15%, position 0.5 to 2% of frame height |

## How

Analysis in JS with Web Audio; pass the four smoothed values and the kick envelope to the
shader as uniforms.

```js
class Envelope {
  constructor(attack, release) { this.a = attack; this.r = release; this.v = 0; }
  step(x, dt) {
    const tau = x > this.v ? this.a : this.r;
    this.v += (x - this.v) * (1 - Math.exp(-dt / tau));
    return this.v;
  }
}
class AutoGain {
  constructor(decay) { this.decay = decay; this.peak = 1e-6; }
  step(x, dt) {
    this.peak = Math.max(x, this.peak * Math.exp(-dt / this.decay));
    return x / this.peak;
  }
}

async function startAudio() {
  const ctx = new AudioContext(); // call from a click handler: browsers need a gesture
  const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
  const analyser = ctx.createAnalyser();
  analyser.fftSize = 2048;
  analyser.smoothingTimeConstant = 0; // we smooth ourselves
  ctx.createMediaStreamSource(stream).connect(analyser);
  const db = new Float32Array(analyser.frequencyBinCount);
  const bin = (hz) => Math.round((hz * analyser.fftSize) / ctx.sampleRate);
  const band = (lo, hi) => {
    let s = 0;
    for (let i = bin(lo); i <= bin(hi); i++) s += Math.pow(10, db[i] / 10);
    return s / (bin(hi) - bin(lo) + 1);
  };

  const bands = [[30, 120], [120, 300], [300, 2000], [6000, 14000]];
  const gains = bands.map(() => new AutoGain(8));
  const envs = [new Envelope(0.01, 0.25), new Envelope(0.05, 0.8),
                new Envelope(0.03, 0.4), new Envelope(0.01, 0.15)];
  const kick = new Envelope(0.005, 0.3);
  let prevLow = 0, mean = 0, varSum = 0, since = 1;

  return function analyse(dt) {
    analyser.getFloatFrequencyData(db);
    const out = bands.map(([lo, hi], i) => envs[i].step(gains[i].step(band(lo, hi), dt), dt));
    const low = gains[0].peak > 0 ? band(30, 120) / gains[0].peak : 0;
    const flux = Math.max(0, low - prevLow);
    prevLow = low;
    mean += (flux - mean) * 0.05;
    varSum += ((flux - mean) * (flux - mean) - varSum) * 0.05;
    since += dt;
    const onset = flux > mean + 1.5 * Math.sqrt(varSum) && since > 0.12;
    if (onset) since = 0;
    return { low: out[0], body: out[1], mid: out[2], air: out[3],
             kick: kick.step(onset ? 1 : 0, dt), onset };
  };
}
```

In Shadertoy, the audio texture's first row is an already-smoothed spectrum, linear in
frequency; bass is roughly the first 2% of its width:

```glsl
float fftBand(float lo, float hi) {
    float s = 0.0;
    for (int i = 0; i < 8; i++) {
        float x = mix(lo, hi, (float(i) + 0.5) / 8.0);
        s += texture(iChannel0, vec2(x, 0.25)).x;
    }
    return s / 8.0;
}
// float bass = smoothstep(0.35, 0.85, fftBand(0.0, 0.02));
```

## Don't

- Don't map volume to the scale of the whole frame: [jitter](../rule/jitter.md).
- Don't let more than one element class pulse on the kick.
- Don't skip auto-gain; levels differ by 20 dB between venues and tracks.

## Check

- On a steady four-on-the-floor track, the kick value rises once per beat and is near 0
  between beats, with no flicker.
- With silence, nothing moves except ambient drift.

## Next

- [Sync](../vj/vj-sync.md)
- [Rhythm](../principle/rhythm.md)
