//! Motion graphics and VJ knowledge graph, embedded from `knowledge/` at build time.

use agent_man::{CategoryConfig, DocEngine, DocPage};

pub const BIN: &str = "agent-man-mg";

const PAGES: &[(&str, &str)] = &include!(concat!(env!("OUT_DIR"), "/agent_man_pages.rs"));

/// Build the engine with every embedded page and the category order the index uses.
pub fn engine() -> DocEngine {
    let mut engine = DocEngine::new(BIN)
        .with_version(env!("CARGO_PKG_VERSION"))
        .with_subcommands("guide", "explain")
        .with_scheme("mg")
        .with_categories(vec![
            CategoryConfig::new("start", "Start here"),
            CategoryConfig::new("process", "Process (follow in order)"),
            CategoryConfig::new("ideation", "Phase 1: finding the idea"),
            CategoryConfig::new("idea", "Ways of thinking (Phase 1, pick by the question)"),
            CategoryConfig::new("principle", "Principles"),
            CategoryConfig::new("table", "Reference tables (numbers)"),
            CategoryConfig::new("look", "Looks (coherent style bundles)"),
            CategoryConfig::new("recipe", "Recipes (code)"),
            CategoryConfig::new("vj", "VJ and live performance"),
            CategoryConfig::new("example", "Worked examples (brief to finished code)"),
            CategoryConfig::new("rule", "Critique: symptom to fix (use `explain`)")
                .with_id_as_title(true),
        ]);
    engine.add_pages(PAGES.iter().copied());
    engine
}

#[derive(Debug, Clone, PartialEq)]
pub struct Hit {
    pub id: String,
    pub category: String,
    pub summary: String,
    pub score: u32,
}

const STOPWORDS: &[&str] = &[
    "a", "an", "and", "are", "as", "at", "be", "by", "do", "for", "from", "how", "i", "in", "is",
    "it", "make", "my", "of", "on", "or", "should", "the", "to", "what", "when", "with",
];

fn terms(query: &str) -> Vec<String> {
    query
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .filter(|t| !t.is_empty() && !STOPWORDS.contains(t))
        .map(str::to_string)
        .collect()
}

fn score_page(page: &DocPage, terms: &[String]) -> u32 {
    let id = page.id.to_lowercase();
    let title = page.title.to_lowercase();
    let summary = page.summary.to_lowercase();
    let body = page.raw_content.to_lowercase();
    let aliases: Vec<String> = page.aliases.iter().map(|a| a.to_lowercase()).collect();

    let mut total = 0;
    let mut matched = 0;
    for t in terms {
        let mut s = 0;
        if id == *t || aliases.iter().any(|a| a == t) {
            s += 8;
        }
        if title.contains(t.as_str()) {
            s += 5;
        }
        if aliases.iter().any(|a| a.contains(t.as_str())) {
            s += 4;
        }
        if summary.contains(t.as_str()) {
            s += 3;
        }
        s += body.matches(t.as_str()).count().min(5) as u32;
        if s > 0 {
            matched += 1;
        }
        total += s;
    }
    // A page that matches more of the words outranks one that repeats a single word.
    total * matched
}

/// Full-text search over id, title, aliases, summary and body. Best first.
pub fn search(engine: &DocEngine, query: &str, category: Option<&str>, limit: usize) -> Vec<Hit> {
    let terms = terms(query);
    if terms.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<Hit> = engine
        .index
        .pages
        .iter()
        .filter(|p| category.is_none_or(|c| p.category.eq_ignore_ascii_case(c)))
        .map(|p| Hit {
            id: p.id.clone(),
            category: p.category.clone(),
            summary: p.summary.clone(),
            score: score_page(p, &terms),
        })
        .filter(|h| h.score > 0)
        .collect();
    hits.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.id.cmp(&b.id)));
    hits.truncate(limit);
    hits
}

/// The command an agent runs to read a page: `explain` for critique rules, `guide` otherwise.
pub fn command_for(id: &str, category: &str) -> String {
    let sub = if category == "rule" { "explain" } else { "guide" };
    format!("{BIN} {sub} {id}")
}

pub fn render_hits(hits: &[Hit]) -> String {
    let mut out = String::new();
    for h in hits {
        out.push_str(&format!(
            "  * {} [{}]: {} (`{}`)\n",
            h.id,
            h.category,
            h.summary,
            command_for(&h.id, &h.category)
        ));
    }
    out
}
