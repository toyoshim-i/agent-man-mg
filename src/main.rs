use std::io::IsTerminal;
use std::process::ExitCode;

use agent_man::mcp::{ParamType, Tool, ToolResult};
use agent_man::{DocEngine, RenderMode};
use agent_man_mg::{engine, render_hits, search, BIN};

const HELP: &str = "\
agent-man-mg: motion graphics and VJ knowledge for agents

START HERE:  agent-man-mg guide start
  It is a step-by-step procedure in two phases: finding the idea (ways of
  thinking from earlier work, held against today), then the craft. Follow it
  in order; every page ends with the pages to read next.

Commands:
  guide                 list every topic, grouped by kind
  guide <topic>         read a topic (id, title or alias; partial match)
  explain <symptom>     diagnose a problem you see in your own output
                        (e.g. banding, jitter, linear-motion, muddy-palette)
  search <words...>     full-text search when you do not know the topic id
  --mcp                 serve the same knowledge over MCP stdio
  --version             print the version
";

fn mode() -> RenderMode {
    if std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none() {
        RenderMode::Terminal
    } else {
        RenderMode::Agent
    }
}

/// Print a page, or the closest pages when the lookup misses.
fn read_or_suggest(
    engine: &DocEngine,
    query: &str,
    result: Result<String, agent_man::DocError>,
    category: Option<&str>,
) -> ExitCode {
    match result {
        Ok(text) => {
            print!("{text}");
            ExitCode::SUCCESS
        }
        Err(_) => {
            let hits = search(engine, query, category, 8);
            if hits.is_empty() {
                eprintln!("No topic matches '{query}'. Run '{BIN} guide' for the index.");
                return ExitCode::FAILURE;
            }
            println!("No exact topic '{query}'. Closest matches:\n");
            print!("{}", render_hits(&hits));
            ExitCode::SUCCESS
        }
    }
}

fn serve_mcp(mut engine: DocEngine) -> ExitCode {
    let index = engine.clone();
    let tool = Tool::builder(
        "search",
        "Full-text search of the motion graphics knowledge graph. Returns page ids to read with doc_read.",
    )
    .param("query", ParamType::String, "Words describing what you need", true)
    .see("start")
    .handler(move |args| {
        let q = args["query"].as_str().unwrap_or("");
        let hits = search(&index, q, None, 8);
        if hits.is_empty() {
            ToolResult::text(format!("No pages match '{q}'. Call doc_list for the index.")).into()
        } else {
            ToolResult::text(render_hits(&hits)).into()
        }
    })
    .build();
    if let Err(e) = engine.register_tool(tool) {
        eprintln!("{e}");
        return ExitCode::FAILURE;
    }
    match engine.serve_stdio() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let engine = engine();
    let rest = args.get(1..).unwrap_or(&[]).join(" ");
    let rest = rest.trim();

    match args.first().map(String::as_str) {
        Some("guide") if rest.is_empty() => {
            print!("{}", engine.guide(None, mode()).unwrap_or_default());
            println!("Start with '{BIN} guide start'.");
            ExitCode::SUCCESS
        }
        Some("guide") => read_or_suggest(&engine, rest, engine.guide(Some(rest), mode()), None),
        Some("explain") if !rest.is_empty() => {
            read_or_suggest(&engine, rest, engine.explain(rest, mode()), Some("rule"))
        }
        Some("search") if !rest.is_empty() => {
            let hits = search(&engine, rest, None, 10);
            if hits.is_empty() {
                eprintln!("No pages match '{rest}'. Run '{BIN} guide' for the index.");
                return ExitCode::FAILURE;
            }
            print!("{}", render_hits(&hits));
            ExitCode::SUCCESS
        }
        Some("--mcp") | Some("mcp") => serve_mcp(engine),
        Some("--version") | Some("-V") => {
            println!("{BIN} {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        None | Some("help") | Some("--help") | Some("-h") => {
            print!("{HELP}");
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("Unknown command '{other}'.\n\n{HELP}");
            ExitCode::FAILURE
        }
    }
}
