---
id: example-kinetic-quote
title: Example - kinetic quote (HTML)
summary: Use when animating a quote, slogan, lyric or message as kinetic typography. A complete worked example with two cards, an accent word, a secondary rule line and frozen-time capture.
category: example
aliases: [kinetic-quote, typography-animation, slogan, lyric-video, text-motion]
---
# Example: kinetic quote

Every step of [the procedure](../start/start.md), with its written output.

## 1. Brief

```text
purpose:     a design studio's line lands as a belief, not a slogan
audience:    studio website hero and social clip (16:9 here; 9:16 variant later)
medium:      HTML/CSS/JS
format:      1920x1080, browser
duration:    8.5 s
content:     card 1 "Every frame / is a decision."  card 2 "Make each / one count."
music:       none
mood words:  confident, precise, calm
must avoid:  bouncy or cute motion
done when:   each card is fully readable and still for its read time; 160 px thumbnail legible
```

## 2. Concept

Purpose as a question: "How does a sentence land as a belief, in a feed full of animated text?"
Three ideas from three families:

```text
idea:          type-as-image (letters)
then asked:    what if letters are seen before they are read?
now asks:      auto-captions put neutral text on every video; what gives words a voice?
became free:   animated text on screen
became scarce: letterforms and layout with conviction
opening:       one heavy face, set tight, with breaks chosen for weight

idea:          stillness-as-motion (constraint and medium)
then asked:    how can holding still make motion stronger?
now asks:      every caption slides, bounces and wiggles; what is a word that lands?
became free:   smooth, continuous text motion
became scarce: the hold; motion that hits and stops
opening:       words drop into place fast, then hold completely still

idea:          ma (time and meaning)
then asked:    what if the empty interval is the expression?
now asks:      clips fill every second; can a pause carry the claim?
became free:   constant motion
became scarce: silence between statements
opening:       a long empty beat between the two cards
```

| Concept | Purpose | Now | Glance | Build | Mood | Total |
|---|---|---|---|---|---|---|
| "Words drop into their lines like set metal type, hold still, and one word is ruled" | 2 | 1 | 2 | 2 | 2 | 9 |
| "Letters assemble from scattered glyphs" | 1 | 0 | 1 | 2 | 1 | out (0 on now) |
| "The first card, then 2 s of empty black, then the second" | 1 | 1 | 2 | 2 | 1 | 7 |

Chosen: the metal-type drop. From *ma*, the exit of card 1 and the entrance of card 2 are
separated by a clean gap, not overlapped.

## 3. Design sheet

```text
look:      swiss, dark
palette:   ink-night: bg #0E0F13, main #E9E6DF, accent #FF5A36 (one word + rule line)
type:      Inter Tight 800, 150 px (1/7 height), tracking -3%, line height 1.0, flush left col 2
motion:    mask rise 0.7 s outExpo, word stagger 0.07 s jittered, exit 0.45 s inExpo upward
           rule line draws 0.5 s outExpo, 0.15 s after its word lands (secondary)
texture:   none
camera:    static
```

## 4. Structure

Read time per card: 5 words -> `max(1.5, 5 / 3 + 1.0)` = 2.67 s.

```text
t_start  dur    element            ease
0.30     0.97   card 1 words in         outExpo, stagger 0.07 x jitter; last lands 1.27
1.42     0.50   rule under "decision."  outExpo (secondary, 0.15 s after its word)
1.27     2.67   card 1 hold             -
3.94     0.61   card 1 out              inExpo, stagger 0.04
4.55     0.30   (ma)                    empty frame between the cards
4.85     0.91   card 2 words in         outExpo; last lands 5.76
5.91     0.50   rule under "count."     outExpo
5.76     2.74   card 2 hold, to 8.5 s   read time for 4 words is 2.33 s
```

## 5. Build

```html
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<link href="https://fonts.googleapis.com/css2?family=Inter+Tight:wght@800&display=swap" rel="stylesheet">
<style>
  html, body { margin: 0; height: 100%; background: #0E0F13; overflow: hidden; }
  #stage { position: absolute; width: 1920px; height: 1080px; left: 50%; top: 50%;
           transform-origin: 0 0; }
  .card { position: absolute; left: 330px; top: 50%; transform: translateY(-50%); margin: 0;
          font: 800 150px/1.0 "Inter Tight", sans-serif; letter-spacing: -0.03em; color: #E9E6DF; }
  .line { overflow: hidden; padding-bottom: 0.1em; }
  .word { display: inline-block; margin-right: 0.22em; position: relative; }
  .accent { color: #FF5A36; }
  .rule { position: absolute; left: 0; right: 0.1em; bottom: -0.02em; height: 0.06em;
          background: #FF5A36; transform-origin: 0 50%; transform: scaleX(0); }
</style>
</head>
<body>
<div id="stage"></div>
<script>
const OUT_EXPO = "cubic-bezier(0.16, 1, 0.3, 1)";
const IN_EXPO = "cubic-bezier(0.7, 0, 0.84, 0)";
const CARDS = [
  { lines: [["Every", "frame"], ["is", "a", "*decision.*"]], tIn: 0.3 },
  { lines: [["Make", "each"], ["one", "*count.*"]], tIn: 4.85, last: true },
];
const D_IN = 0.7, STAGGER = 0.07, D_OUT = 0.45, D_RULE = 0.5;
const jitter = [1.0, 0.8, 1.2, 0.9, 1.1, 1.0]; // fixed, so the stagger is not mechanical

const stage = document.getElementById("stage");
const fit = () => {
  const s = Math.min(innerWidth / 1920, innerHeight / 1080);
  stage.style.transform = `scale(${s}) translate(-50%, -50%)`;
};
addEventListener("resize", fit);
fit();

const ms = (s) => s * 1000;
for (const card of CARDS) {
  const el = document.createElement("h1");
  el.className = "card";
  stage.appendChild(el);
  const words = [];
  for (const line of card.lines) {
    const l = document.createElement("div");
    l.className = "line";
    for (const w of line) {
      const span = document.createElement("span");
      span.className = "word";
      const isAccent = w.startsWith("*");
      span.textContent = isAccent ? w.slice(1, -1) : w;
      if (isAccent) {
        span.classList.add("accent");
        const rule = document.createElement("i");
        rule.className = "rule";
        span.appendChild(rule);
      }
      l.appendChild(span);
      words.push(span);
    }
    el.appendChild(l);
  }
  let t = card.tIn;
  const landed = [];
  words.forEach((w, i) => {
    w.animate([{ transform: "translateY(110%)" }, { transform: "translateY(0)" }],
      { duration: ms(D_IN), delay: ms(t), easing: OUT_EXPO, fill: "both" });
    landed.push(t + D_IN);
    t += STAGGER * jitter[i % jitter.length];
  });
  const lastLand = Math.max(...landed);
  const rule = el.querySelector(".rule");
  rule.animate([{ transform: "scaleX(0)" }, { transform: "scaleX(1)" }],
    { duration: ms(D_RULE), delay: ms(lastLand + 0.15), easing: OUT_EXPO, fill: "both" });
  if (!card.last) {
    const hold = Math.max(1.5, words.length / 3 + 1.0);
    const tOut = lastLand + hold;
    words.forEach((w, i) => {
      w.animate([{ transform: "translateY(0)" }, { transform: "translateY(-110%)" }],
        { duration: ms(D_OUT), delay: ms(tOut + i * 0.04), easing: IN_EXPO,
          fill: "forwards", composite: "add" });
    });
  }
}

const frozen = new URLSearchParams(location.search).get("t");
if (frozen !== null) {
  document.fonts.ready.then(() => {
    for (const a of document.getAnimations()) { a.pause(); a.currentTime = ms(parseFloat(frozen)); }
  });
}
</script>
</body>
</html>
```

## 6. Review

| Check | Expected |
|---|---|
| Curves | every move uses outExpo in, inExpo out |
| Stagger | gaps vary 0.056 to 0.084 s, by the jitter table |
| One star | words land, then the rule line draws: never both as primaries |
| Read time | card 1 is still for 2.67 s, card 2 for 2.74 s (needs 2.33) |
| Thumbnail | 150 px type at 160 px wide is about 12 px tall: legible |
| Capture | `?t=2.0` and `?t=6.5` show the full cards |

## Next

- [Logo sting example](example-logo-sting.md)
- [Typography](../principle/typography.md)
