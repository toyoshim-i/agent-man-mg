//! The knowledge graph is the product. These tests hold its shape: every page is reachable
//! from `start`, follows the page contract (sections per category, `Use when` summaries, size limits), and
//! every GLSL block compiles.

use std::collections::{HashSet, VecDeque};
use std::path::Path;


use agent_man_mg::{engine, search};

const CATEGORIES: &[&str] = &[
    "start", "process", "ideation", "idea", "principle", "table", "look", "recipe", "vj", "example",
    "rule",
];

/// Relative link targets in a page body, in order.
fn links(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(i) = rest.find("](") {
        let after = &rest[i + 2..];
        match after.find(')') {
            Some(j) => {
                let target = &after[..j];
                if !target.starts_with("http") {
                    out.push(target.to_string());
                }
                rest = &after[j..];
            }
            None => break,
        }
    }
    out
}

#[test]
fn no_broken_links() {
    let broken = engine().broken_links();
    assert!(broken.is_empty(), "broken links: {broken:#?}");
}

#[test]
fn every_page_is_reachable_from_start() {
    let e = engine();
    let mut seen = HashSet::from(["start".to_string()]);
    let mut queue = VecDeque::from(["start".to_string()]);
    while let Some(id) = queue.pop_front() {
        let page = e.lookup_exact(&id).expect("page in queue exists");
        for target in links(&page.raw_content) {
            if let Some(link) = e.index.resolve_link(&page.path, &target) {
                if seen.insert(link.target_id.clone()) {
                    queue.push_back(link.target_id);
                }
            }
        }
    }
    let unreachable: Vec<&str> = e
        .index
        .pages
        .iter()
        .map(|p| p.id.as_str())
        .filter(|id| !seen.contains(*id))
        .collect();
    assert!(unreachable.is_empty(), "not reachable from start: {unreachable:?}");
}

#[test]
fn pages_follow_the_contract() {
    let e = engine();
    let mut ids = HashSet::new();
    let mut problems = Vec::new();
    for p in &e.index.pages {
        if !ids.insert(p.id.to_lowercase()) {
            problems.push(format!("{}: duplicate id", p.id));
        }
        if !CATEGORIES.contains(&p.category.as_str()) {
            problems.push(format!("{}: unknown category '{}'", p.id, p.category));
        }
        if !p.summary.starts_with("Use when") {
            problems.push(format!("{}: summary must start with 'Use when'", p.id));
        }
        let lines = p.source_content.lines().count();
        let limit = if p.category == "example" { 280 } else { 160 };
        if lines > limit {
            problems.push(format!("{}: {lines} lines, limit is {limit}", p.id));
        }
        let headings: Vec<&str> = p.sections.iter().map(|s| s.heading.as_str()).collect();
        let required: &[&str] = match p.category.as_str() {
            "rule" => &["Symptom", "Cause", "Fix", "Next"],
            "principle" | "recipe" => &["Rule", "Defaults", "Don't", "Check", "Next"],
            "idea" => &["Question", "Then", "On screen", "Now", "Openings", "Next"],
            _ => &["Next"],
        };
        for r in required {
            if !headings.contains(r) {
                problems.push(format!("{}: missing '## {r}'", p.id));
            }
        }
        if links(&p.raw_content).is_empty() {
            problems.push(format!("{}: dead end, no outbound links", p.id));
        }
    }
    assert!(problems.is_empty(), "contract violations:\n{}", problems.join("\n"));
}

/// Shadertoy-style prelude so a block that defines `mainImage` compiles as a fragment shader.
const PRELUDE: &str = "#version 450
layout(location = 0) out vec4 _fragColor;
layout(set = 0, binding = 0) uniform Globals {
    vec3 iResolution;
    float iTime;
    vec4 iMouse;
    float iTimeDelta;
    int iFrame;
};
layout(set = 0, binding = 1) uniform texture2D _ch0;
layout(set = 0, binding = 2) uniform texture2D _ch1;
layout(set = 0, binding = 3) uniform sampler _smp;
#define iChannel0 sampler2D(_ch0, _smp)
#define iChannel1 sampler2D(_ch1, _smp)
";

#[test]
fn glsl_blocks_compile() {
    let e = engine();
    let mut failures = Vec::new();
    let mut count = 0;
    for block in e.code_blocks("glsl") {
        count += 1;
        let main = if block.source.contains("void mainImage") {
            "\nvoid main() { vec4 c; mainImage(c, gl_FragCoord.xy); _fragColor = c; }\n"
        } else {
            "\nvoid main() { _fragColor = vec4(0.0); }\n"
        };
        let src = format!("{PRELUDE}{}{main}", block.source);
        let mut frontend = naga::front::glsl::Frontend::default();
        let options = naga::front::glsl::Options::from(naga::ShaderStage::Fragment);
        let module = match frontend.parse(&options, &src) {
            Ok(m) => m,
            Err(err) => {
                failures.push(format!("{}:{} parse: {err:?}", block.path, block.line));
                continue;
            }
        };
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        if let Err(err) = validator.validate(&module) {
            failures.push(format!("{}:{} validate: {err:?}", block.path, block.line));
        }
    }
    assert!(count > 0, "no glsl blocks found");
    assert!(failures.is_empty(), "{} glsl blocks failed:\n{}", failures.len(), failures.join("\n"));
}

/// JS blocks, and the inline script of HTML blocks, must parse. Skipped without node.
#[test]
fn js_blocks_parse() {
    if std::process::Command::new("node").arg("--version").output().is_err() {
        eprintln!("node not found; skipping");
        return;
    }
    let e = engine();
    let dir = std::env::temp_dir().join(format!("agent-man-mg-js-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut sources = Vec::new();
    for block in e.code_blocks("js") {
        sources.push((format!("{}:{}", block.path, block.line), block.source));
    }
    for block in e.code_blocks("html") {
        let s = block.source.find("<script>").map(|i| i + "<script>".len());
        let end = block.source.find("</script>");
        if let (Some(s), Some(end)) = (s, end) {
            sources.push((format!("{}:{}", block.path, block.line), block.source[s..end].to_string()));
        }
    }
    assert!(!sources.is_empty(), "no js or html blocks found");
    let mut failures = Vec::new();
    for (i, (origin, src)) in sources.iter().enumerate() {
        let file = dir.join(format!("block{i}.js"));
        std::fs::write(&file, src).unwrap();
        let out = std::process::Command::new("node").arg("--check").arg(&file).output().unwrap();
        if !out.status.success() {
            failures.push(format!("{origin}: {}", String::from_utf8_lossy(&out.stderr)));
        }
    }
    std::fs::remove_dir_all(&dir).ok();
    assert!(failures.is_empty(), "js blocks failed:\n{}", failures.join("\n"));
}

#[test]
fn search_finds_by_body_words() {
    let e = engine();
    let hits = search(&e, "banding gradient", None, 5);
    assert!(hits.iter().any(|h| h.id == "banding"), "{hits:?}");
}

#[test]
fn skill_md_is_fresh() {
    let e = engine();
    let skill = e.skill_md(
        "agent-man-mg",
        "Motion graphics and VJ craft knowledge. Use before designing, animating, writing shaders or critiquing visuals. Run `agent-man-mg guide start` and follow it.",
    );
    agent_man::assert_or_bless(Path::new("SKILL.md"), &skill).unwrap();
}
