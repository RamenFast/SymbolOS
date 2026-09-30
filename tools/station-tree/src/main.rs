//! station-tree: checks the grown SymbolOS docs.
//! lint   fence width, header width, tables, dashes, semicolons
//! check  NODE/STATE pairs against a concourse status JSON
//! links  relative markdown links resolve on disk
//! all    lint + check + links
//! schema the contract
use serde_json::{json, Value};
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

const TOOL: &str = "station-tree";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const FENCE_MAX: usize = 38;
const HEADER_MAX: usize = 34;
const TABLE_MAX_COLS: usize = 3;
const AGENT_STATES: [&str; 3] = ["running", "ancestor", "present"];

struct Finding {
    file: String,
    line: usize,
    kind: &'static str,
    msg: String,
}

fn ts() -> String {
    let out = Command::new("date").arg("+%Y-%m-%dT%H:%M:%S%:z").output();
    match out {
        Ok(o) => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        Err(_) => "unknown".into(),
    }
}

fn envelope(status: &str) -> serde_json::Map<String, Value> {
    let mut m = serde_json::Map::new();
    m.insert("status".into(), json!(status));
    m.insert("tool".into(), json!(TOOL));
    m.insert("version".into(), json!(VERSION));
    m.insert("ts".into(), json!(ts()));
    m
}

fn fail(code: i32, err: &str, fix: &str, want_json: bool) -> ! {
    if want_json {
        let mut m = envelope("error");
        m.insert("error".into(), json!(err));
        m.insert("fix".into(), json!(fix));
        println!("{}", Value::Object(m));
    } else {
        eprintln!("error: {err}\nfix: {fix}");
    }
    exit(code)
}

fn strip_code_spans(s: &str) -> String {
    let mut out = String::new();
    let mut in_span = false;
    for c in s.chars() {
        if c == '`' {
            in_span = !in_span;
            continue;
        }
        if !in_span {
            out.push(c);
        }
    }
    out
}

fn lint(file: &str, text: &str, out: &mut Vec<Finding>) {
    let mut in_fence = false;
    let f = || file.to_string();
    for (i, raw) in text.lines().enumerate() {
        let n = i + 1;
        let line = raw.trim_end_matches('\r');
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        let width = line.chars().count();
        if in_fence {
            let is_header = line.starts_with('║') || line.starts_with('╔') || line.starts_with('╚');
            let max = if is_header { HEADER_MAX } else { FENCE_MAX };
            if width > max {
                out.push(Finding { file: f(), line: n, kind: "width", msg: format!("{width} chars, max {max}: {line}") });
            }
            if line.contains('—') && !line.contains("— Rhy") {
                out.push(Finding { file: f(), line: n, kind: "dash", msg: "em dash outside Rhy's signature".into() });
            }
            continue;
        }
        if line.starts_with('|') {
            let cols = line.trim_matches('|').split('|').count();
            if cols > TABLE_MAX_COLS {
                out.push(Finding { file: f(), line: n, kind: "table", msg: format!("{cols} columns, max {TABLE_MAX_COLS}") });
            }
        }
        let quoted = line.trim_start().starts_with('>');
        if line.contains('—') && !line.contains("— Rhy") && !quoted {
            out.push(Finding { file: f(), line: n, kind: "dash", msg: "em dash in prose".into() });
        }
        let prose = strip_code_spans(line);
        if prose.contains(';') && !quoted && !prose.contains("&") {
            out.push(Finding { file: f(), line: n, kind: "semicolon", msg: "semicolon in prose".into() });
        }
    }
    if in_fence {
        out.push(Finding { file: f(), line: text.lines().count(), kind: "fence", msg: "unclosed code fence".into() });
    }
}

fn load_status(path: Option<&str>, live: bool, want_json: bool) -> (String, Value) {
    if live {
        let o = Command::new("concourse").args(["status", "--json"]).output();
        let Ok(o) = o else {
            fail(2, "concourse not on PATH", "install concourse or pass --status <file>", want_json)
        };
        let v: Value = serde_json::from_slice(&o.stdout)
            .unwrap_or_else(|e| fail(4, &format!("concourse status --json did not parse: {e}"), "run concourse status --json by hand", want_json));
        return ("concourse status --json".into(), v);
    }
    let p = match path {
        Some(p) => PathBuf::from(p),
        None => find_status(want_json),
    };
    let text = fs::read_to_string(&p)
        .unwrap_or_else(|e| fail(4, &format!("cannot read {}: {e}", p.display()), "pass --status <file> or --live", want_json));
    let v: Value = serde_json::from_str(&text)
        .unwrap_or_else(|e| fail(4, &format!("{} is not JSON: {e}", p.display()), "regenerate with concourse status --json", want_json));
    (p.display().to_string(), v)
}

fn find_status(want_json: bool) -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        let mut best: Option<PathBuf> = None;
        if let Ok(rd) = fs::read_dir(&dir) {
            for e in rd.flatten() {
                let name = e.file_name().to_string_lossy().to_string();
                if name.starts_with(".station-status-") && name.ends_with(".json") {
                    if best.as_ref().map_or(true, |b| e.path() > *b) {
                        best = Some(e.path());
                    }
                }
            }
        }
        if let Some(b) = best {
            return b;
        }
        if !dir.pop() {
            fail(2, "no .station-status-*.json found in cwd or parents", "pass --status <file> or --live", want_json)
        }
    }
}

fn check(file: &str, text: &str, states: &serde_json::Map<String, Value>, seen: &mut Vec<String>, out: &mut Vec<Finding>) {
    let lines: Vec<&str> = text.lines().collect();
    let mut in_fence = false;
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        if l.trim_start().starts_with("```") {
            in_fence = !in_fence;
            i += 1;
            continue;
        }
        if !in_fence {
            i += 1;
            continue;
        }
        if let Some(rest) = l.strip_prefix("NODE") {
            let node = rest.trim().to_string();
            if node.starts_with('<') {
                i += 1;
                continue;
            }
            let mut state: Option<String> = None;
            for j in 1..=3 {
                if let Some(s) = lines.get(i + j).and_then(|x| x.strip_prefix("STATE")) {
                    state = Some(s.trim().to_string());
                    break;
                }
            }
            let n = i + 1;
            match (node.as_str(), state) {
                (_, None) => out.push(Finding { file: file.into(), line: n, kind: "block", msg: format!("NODE {node} has no STATE within 3 lines") }),
                ("none", Some(st)) => {
                    if !AGENT_STATES.contains(&st.as_str()) {
                        out.push(Finding { file: file.into(), line: n, kind: "state", msg: format!("NODE none allows {:?}, got {st}", AGENT_STATES) });
                    }
                }
                (id, Some(st)) => match states.get(id) {
                    None => out.push(Finding { file: file.into(), line: n, kind: "node", msg: format!("{id} is not in the status JSON") }),
                    Some(truth) => {
                        seen.push(id.to_string());
                        let truth = truth.as_str().unwrap_or("");
                        if truth != st {
                            out.push(Finding { file: file.into(), line: n, kind: "state", msg: format!("{id}: doc says {st}, station says {truth}") });
                        }
                    }
                },
            }
        }
        i += 1;
    }
}

fn links(file: &str, text: &str, out: &mut Vec<Finding>) {
    let base = Path::new(file).parent().unwrap_or(Path::new("."));
    for (i, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else { break };
            let target = &after[..end];
            rest = &after[end + 1..];
            if target.starts_with("http://") || target.starts_with("https://") || target.starts_with("mailto:") || target.starts_with('#') || target.is_empty() {
                continue;
            }
            let path = target.split('#').next().unwrap_or("");
            let p = if path.starts_with('/') { PathBuf::from(path.trim_start_matches('/')) } else { base.join(path) };
            if !p.exists() {
                out.push(Finding { file: file.into(), line: i + 1, kind: "link", msg: format!("{target} does not resolve") });
            }
        }
    }
}

fn schema() -> Value {
    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "station-tree",
        "description": "Checks SymbolOS grown docs against the Pattern Room forms and the live station.",
        "verbs": {
            "lint":   {"args": ["<file>..."], "flags": ["--json"], "checks": ["width", "dash", "semicolon", "table", "fence"]},
            "check":  {"args": ["<file>..."], "flags": ["--json", "--status <file>", "--live"], "checks": ["node", "state", "block"]},
            "links":  {"args": ["<file>..."], "flags": ["--json"], "checks": ["link"]},
            "all":    {"args": ["<file>..."], "flags": ["--json", "--status <file>", "--live"]},
            "schema": {"args": [], "flags": []}
        },
        "limits": {"fence_max": FENCE_MAX, "header_max": HEADER_MAX, "table_max_cols": TABLE_MAX_COLS},
        "agent_states": AGENT_STATES,
        "exit_codes": {"0": "clean", "2": "status source unavailable", "3": "bad arguments", "4": "findings or runtime failure"},
        "envelope": {"type": "object", "additionalProperties": false, "required": ["status", "tool", "version", "ts"],
            "properties": {
                "status": {"enum": ["ok", "error"]}, "tool": {"const": TOOL}, "version": {"type": "string"}, "ts": {"type": "string"},
                "source": {"type": "string", "description": "status JSON path or the live command"},
                "files": {"type": "integer"}, "nodes_seen": {"type": "integer"}, "nodes_total": {"type": "integer"},
                "nodes_unmapped": {"type": "array", "items": {"type": "string"}},
                "findings": {"type": "array", "items": {"type": "object", "additionalProperties": false,
                    "required": ["file", "line", "kind", "msg"],
                    "properties": {"file": {"type": "string"}, "line": {"type": "integer"},
                        "kind": {"enum": ["width", "dash", "semicolon", "table", "fence", "node", "state", "block", "link"]},
                        "msg": {"type": "string"}}}},
                "error": {"type": "string"}, "fix": {"type": "string"}
            }}
    })
}

fn usage() -> &'static str {
    "usage: station-tree <lint|check|links|all> <file>... [--json] [--status <file>] [--live]\n       station-tree schema"
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut want_json = !std::io::stdout().is_terminal();
    let mut live = false;
    let mut status_path: Option<String> = None;
    let mut files: Vec<String> = Vec::new();
    let mut verb: Option<String> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--json" => want_json = true,
            "--live" => live = true,
            "--status" => status_path = it.next().cloned(),
            _ if verb.is_none() => verb = Some(a.clone()),
            _ => files.push(a.clone()),
        }
    }
    let Some(verb) = verb else { fail(3, "no verb", usage(), want_json) };
    if verb == "schema" {
        let mut m = envelope("ok");
        m.insert("schema".into(), schema());
        println!("{}", serde_json::to_string_pretty(&Value::Object(m)).unwrap());
        return;
    }
    if !["lint", "check", "links", "all"].contains(&verb.as_str()) {
        fail(3, &format!("unknown verb {verb}"), usage(), want_json);
    }
    if files.is_empty() {
        fail(3, "no files given", usage(), want_json);
    }
    let do_lint = verb == "lint" || verb == "all";
    let do_check = verb == "check" || verb == "all";
    let do_links = verb == "links" || verb == "all";

    let mut findings = Vec::new();
    let mut source = String::new();
    let mut states = serde_json::Map::new();
    if do_check {
        let (src, v) = load_status(status_path.as_deref(), live, want_json);
        source = src;
        for n in v["nodes"].as_array().cloned().unwrap_or_default() {
            if let (Some(id), Some(st)) = (n["id"].as_str(), n["state"].as_str()) {
                states.insert(id.into(), json!(st));
            }
        }
    }
    let mut seen: Vec<String> = Vec::new();
    for f in &files {
        let text = fs::read_to_string(f).unwrap_or_else(|e| fail(4, &format!("cannot read {f}: {e}"), "check the path", want_json));
        if do_lint { lint(f, &text, &mut findings); }
        if do_check { check(f, &text, &states, &mut seen, &mut findings); }
        if do_links { links(f, &text, &mut findings); }
    }
    let unmapped: Vec<String> = states.keys().filter(|k| !seen.contains(k)).cloned().collect();

    let clean = findings.is_empty();
    let mut m = envelope(if clean { "ok" } else { "error" });
    if do_check {
        m.insert("source".into(), json!(source));
        m.insert("nodes_seen".into(), json!(seen.len()));
        m.insert("nodes_total".into(), json!(states.len()));
        m.insert("nodes_unmapped".into(), json!(unmapped));
    }
    m.insert("files".into(), json!(files.len()));
    m.insert("findings".into(), Value::Array(findings.iter().map(|f| json!({"file": f.file, "line": f.line, "kind": f.kind, "msg": f.msg})).collect()));
    if !clean {
        m.insert("error".into(), json!(format!("{} findings", findings.len())));
        m.insert("fix".into(), json!("fix each finding at file:line, then rerun"));
    }
    if want_json {
        println!("{}", Value::Object(m));
    } else {
        for f in &findings {
            println!("{}:{}: {}: {}", f.file, f.line, f.kind, f.msg);
        }
        if do_check {
            println!("nodes: {} of {} mapped (source {source})", seen.len(), states.len());
            if !unmapped.is_empty() {
                println!("unmapped: {}", unmapped.join(" "));
            }
        }
        println!("{}: {} files, {} findings", if clean { "clean" } else { "FAIL" }, files.len(), findings.len());
    }
    exit(if clean { 0 } else { 4 })
}
