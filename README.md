# agent-man-mg

Motion graphics and VJ craft knowledge for agents, as a CLI and an MCP server.

The goal: a lightweight model (Haiku, local models) that is given only this tool makes work
at the level of a frontier model. The knowledge comes in two phases:

- **Idea:** ways of thinking behind representative work (visual music, title design, anime
  timing, Vocaloid lyric videos, demoscene, generative art, VJ culture, and more), each held
  against the conditions of today, with a procedure that turns them into a new concept.
- **Craft:** procedures, defaults with numbers, hard limits, paste-ready code and
  symptom-to-fix diagnostics.

Built on [agent-man](https://github.com/toyoshim-i/agent-man): the pages in `knowledge/` are
embedded at build time, and relative links between pages become `agent-man-mg guide <id>`
hints, so an agent walks the graph by following them.

## Use

```bash
cargo install --path .
```

```bash
agent-man-mg guide start
```

| Command | Does |
|---|---|
| `agent-man-mg guide` | index of every page, grouped by kind |
| `agent-man-mg guide <topic>` | read a page (id, title or alias) |
| `agent-man-mg explain <symptom>` | diagnose a defect seen in output: `banding`, `jitter`, `muddy-palette` |
| `agent-man-mg search <words>` | full-text search |
| `agent-man-mg --mcp` | MCP stdio server with `doc_list`, `doc_read` and `search` |

To hand it to an agent as a skill, give it [SKILL.md](SKILL.md).

## Layout

```
knowledge/start      the procedure; the entry point
knowledge/process    brief, concept (phase 1); design sheet, structure, build, review (phase 2)
knowledge/idea       ways of thinking, the ideas index, and the conditions of now
knowledge/principle  timing, easing, color, typography, rhythm, energy, ...
knowledge/table      mood map, durations, BPM, palettes, type pairings
knowledge/look       coherent style bundles
knowledge/recipe     paste-ready GLSL and JS
knowledge/vj         live performance: sets, sync, layers, projection, safety
knowledge/example    brief-to-code worked examples
knowledge/rule       critique: symptom, cause, fix
```

## Tests

```bash
cargo test
```

The tests hold the graph's shape: every page reachable from `start`, the page contract
(required sections per kind of page, `Use when` summaries, size limits), no broken links,
every GLSL block compiles (naga), and every JS and HTML script parses (`node --check`,
skipped without node). After changing pages, refresh SKILL.md with
`AGENT_MAN_BLESS=1 cargo test`.

## License

MIT OR Apache-2.0
