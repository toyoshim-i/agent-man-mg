---
id: kinetic-type-code
title: Kinetic type code (HTML)
summary: Use when animating text in a browser. A self-contained HTML template with mask-rise lines, word and letter stagger, holds and exits, and a frozen-time mode for review.
category: recipe
aliases: [kinetic-type, text-animation, html-template, css-animation, waapi]
---
# Kinetic type code

## Rule

Animate text with the Web Animations API: one animation per word or letter, delays from the
timing chart, easing from the design sheet. Keep the layout static CSS; motion only uses
`transform` and `opacity`. Support `?t=<seconds>` so any frame can be captured.

## Defaults

| Item | Value |
|---|---|
| stage | 1920x1080, scaled to fit the window |
| reveal | mask rise: translateY 105% to 0 inside an `overflow: hidden` line |
| word stagger | 0.07 s |
| reveal duration | 0.7 s, `cubic-bezier(0.16, 1, 0.3, 1)` (outExpo) |
| exit duration | 0.45 s, `cubic-bezier(0.7, 0, 0.84, 0)` (inExpo) |
| hold | `max(1.5, words / 3 + 1.0)` s |

## How

```html
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<link href="https://fonts.googleapis.com/css2?family=Inter+Tight:wght@800&display=swap" rel="stylesheet">
<style>
  html, body { margin: 0; height: 100%; background: #0E0F13; overflow: hidden; }
  #stage { position: absolute; width: 1920px; height: 1080px; left: 50%; top: 50%;
           transform-origin: 0 0; display: flex; flex-direction: column;
           justify-content: center; padding-left: 230px; box-sizing: border-box; }
  .line { overflow: hidden; line-height: 1.0; padding-bottom: 0.08em; }
  .word { display: inline-block; margin-right: 0.22em; will-change: transform; }
  h1 { margin: 0; font: 800 170px/1.0 "Inter Tight", sans-serif; letter-spacing: -0.03em;
       color: #E9E6DF; }
  .accent { color: #FF5A36; }
</style>
</head>
<body>
<div id="stage"><h1 id="text"></h1></div>
<script>
const LINES = [["Make", "it"], ["move", "*with*"], ["intent."]]; // *word* = accent
const OUT_EXPO = "cubic-bezier(0.16, 1, 0.3, 1)";
const IN_EXPO = "cubic-bezier(0.7, 0, 0.84, 0)";
const T_IN = 0.3, D_IN = 0.7, STAGGER = 0.07, D_OUT = 0.45;

const stage = document.getElementById("stage");
function fit() {
  const s = Math.min(innerWidth / 1920, innerHeight / 1080);
  stage.style.transform = `scale(${s}) translate(-50%, -50%)`;
}
addEventListener("resize", fit);
fit();

const text = document.getElementById("text");
const words = [];
for (const line of LINES) {
  const l = document.createElement("div");
  l.className = "line";
  for (const w of line) {
    const span = document.createElement("span");
    span.className = "word";
    const accent = w.startsWith("*");
    if (accent) span.classList.add("accent");
    span.textContent = accent ? w.slice(1, -1) : w;
    l.appendChild(span);
    words.push(span);
  }
  text.appendChild(l);
}

const count = words.length;
const hold = Math.max(1.5, count / 3 + 1.0);
const tOut = T_IN + D_IN + STAGGER * (count - 1) + hold;
words.forEach((el, i) => {
  el.animate(
    [{ transform: "translateY(105%)", opacity: 0 }, { transform: "translateY(0)", opacity: 1 }],
    { duration: D_IN * 1000, delay: (T_IN + i * STAGGER) * 1000, easing: OUT_EXPO, fill: "both" });
  el.animate(
    [{ transform: "translateY(0)" }, { transform: "translateY(-105%)" }],
    { duration: D_OUT * 1000, delay: (tOut + i * STAGGER * 0.6) * 1000, easing: IN_EXPO,
      fill: "forwards", composite: "add" });
});

// ?t=2.5 freezes every animation at 2.5 s for frame capture.
const frozen = new URLSearchParams(location.search).get("t");
if (frozen !== null) {
  document.fonts.ready.then(() => {
    for (const a of document.getAnimations()) { a.pause(); a.currentTime = parseFloat(frozen) * 1000; }
  });
}
</script>
</body>
</html>
```

For letter stagger, split words into letter spans the same way and use 0.03 s per letter.
For Japanese, stagger by word or phrase (bunsetsu), not by character.

## Don't

- Don't animate `top`, `left`, `font-size` or `letter-spacing` every frame for many spans;
  use transforms. (Tracking settle on one line is fine.)
- Don't start the animation before fonts load; the first frames show a fallback font.
- Don't use CSS `transition` for choreography; delays get lost on re-render.

## Check

- `?t=` at the end of the reveal shows every word in place.
- Thumbnail at 160 px wide: still readable.

## Next

- [Typography](../principle/typography.md)
- [Kinetic quote example](../example/example-kinetic-quote.md)
