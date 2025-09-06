use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

use grieg_engine::{ast::to_sexpr, eval::Evaluator, value::V};
use grieg_engine::phase::Phase;
use grieg_parser::parse_expr;

const USAGE: &str = r#"Usage:
  grieg-cli --expr "<expr>" [--ast] [--mem] [--pretty] [--mem-db <file>]
  grieg-cli --repl          [--ast] [--mem] [--pretty] [--mem-db <file>]
  grieg-cli --jsonl <file>  [--ast] [--mem] [--pretty] [--mem-db <file>]

Options:
  --ast                 Include S-expression (AST) in output
  --mem                 Enable MEM semantics
  --pretty              Human-friendly printing (instead of JSON)
  --mem-db <file>       Persist/restore MEM map as JSON
  --manifest            Print build/spec manifest and exit
  -h, --help            Show this help
"#;

fn usage(exit_code: i32) -> ! {
    eprintln!("{USAGE}");
    std::process::exit(exit_code);
}

fn print_manifest_and_exit() -> ! {
    println!(
        "Spec: {} | Grieg: {} | Commit: {} | Target: {} | Built: {}",
        option_env!("SPEC_VERSION").unwrap_or("0.0.0"),
        env!("CARGO_PKG_VERSION"),
        option_env!("GIT_COMMIT").unwrap_or("nogit"),
        option_env!("BUILD_TARGET").unwrap_or("unknown-target"),
        option_env!("BUILD_UNIX_SECS").unwrap_or("0"),
    );
    std::process::exit(0);
}

#[derive(serde::Serialize)]
struct Out<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ast: Option<String>,
    value: serde_json::Value,
    phase: &'a str,
}

/* ---------- emit + mem-db helpers ---------- */

fn emit(out: &Out<'_>, pretty: bool) {
    if pretty {
        let v = match out.value {
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => "unknown".to_string(),
            _ => out.value.to_string(),
        };
        println!(
            "Input: {}\nValue: {}\nPhase: {}\n{}",
            out.input.unwrap_or(""),
            v,
            out.phase,
            match &out.ast {
                Some(s) => format!("AST: {s}"),
                None => String::new(),
            }
        );
    } else {
        println!("{}", serde_json::to_string(out).unwrap());
    }
}

fn load_mem_file(path: &str) -> HashMap<String, bool> {
    if !Path::new(path).exists() {
        return HashMap::new();
    }
    let txt = fs::read_to_string(path).unwrap_or_default();
    serde_json::from_str(&txt).unwrap_or_default()
}

fn save_mem_file(path: &str, map: &HashMap<String, bool>) {
    if let Ok(txt) = serde_json::to_string_pretty(map) {
        let tmp = format!("{}.tmp", path);
        let _ = fs::write(&tmp, txt);
        let _ = fs::rename(tmp, path);
    }
}

/* --------------------------------- main --------------------------------- */

fn main() {
    let mut args = env::args().skip(1);

    let mut expr_arg: Option<String> = None;
    let mut repl = false;
    let mut jsonl: Option<String> = None;
    let mut show_ast = false;
    let mut mem_enabled = false;
    let mut pretty = false;
    let mut mem_db: Option<String> = None;
    let mut show_manifest = false;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--expr"   => { expr_arg = args.next(); }
            "--repl"   => { repl = true; }
            "--jsonl"  => { jsonl = args.next(); }
            "--ast"    => { show_ast = true; }
            "--mem"    => { mem_enabled = true; }
            "--pretty" => { pretty = true; }
            "--mem-db" => {
                mem_db = args.next().or_else(|| {
                    eprintln!("error: --mem-db requires <file>\n");
                    usage(2);
                });
            }
            "--manifest" => { show_manifest = true; }
            "-h" | "--help" => usage(0),
            _ => {}
        }
    }

    if show_manifest { print_manifest_and_exit(); }

    if let Some(path) = jsonl {
        run_jsonl(&path, show_ast, mem_enabled, pretty, &mem_db);
    } else if repl {
        run_repl(show_ast, mem_enabled, pretty, &mem_db);
    } else if let Some(s) = expr_arg {
        run_once(&s, show_ast, mem_enabled, pretty, &mem_db);
    } else {
        usage(2);
    }
}

/* ------------------------------ run modes ------------------------------- */

fn run_once(s: &str, show_ast: bool, mem_enabled: bool, pretty: bool, mem_db: &Option<String>) {
    match parse_expr(s) {
        Ok(ast) => {
            let mut ev = Evaluator::new(mem_enabled);
            if let Some(db) = mem_db {
                ev.import_mem(load_mem_file(db));
            }
            let r = ev.eval(&ast, None);

            let val = match r.value {
                V::Bool(b) => serde_json::json!(b),
                V::Unknown => serde_json::Value::Null,
            };
            let out = Out {
                input: Some(s),
                ast: if show_ast { Some(to_sexpr(&ast)) } else { None },
                value: val,
                phase: match r.phase {
                    Phase::ALIVE => "ALIVE",
                    Phase::JAM   => "JAM",
                    Phase::MEM   => "MEM",
                    Phase::VAC   => "VAC",
                },
            };
            emit(&out, pretty);

            if let Some(db) = mem_db {
                save_mem_file(db, &ev.export_mem());
            }
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
            std::process::exit(2);
        }
    }
}

fn run_jsonl(path: &str, show_ast: bool, mem_enabled: bool, pretty: bool, mem_db: &Option<String>) {
    let file = File::open(path).expect("cannot open file");
    let reader = io::BufReader::new(file);

    let mut ev = Evaluator::new(mem_enabled);
    if let Some(db) = mem_db {
        ev.import_mem(load_mem_file(db));
    }

    for line in reader.lines() {
        let s = line.unwrap();
        if s.trim().is_empty() { continue; }
        match parse_expr(&s) {
            Ok(ast) => {
                let r = ev.eval(&ast, None);
                let val = match r.value {
                    V::Bool(b) => serde_json::json!(b),
                    V::Unknown => serde_json::Value::Null,
                };
                let out = Out {
                    input: Some(&s),
                    ast: if show_ast { Some(to_sexpr(&ast)) } else { None },
                    value: val,
                    phase: match r.phase {
                        Phase::ALIVE => "ALIVE",
                        Phase::JAM   => "JAM",
                        Phase::MEM   => "MEM",
                        Phase::VAC   => "VAC",
                    },
                };
                emit(&out, pretty);
            }
            Err(e) => eprintln!("Parse error: {e}"),
        }
    }

    if let Some(db) = mem_db {
        save_mem_file(db, &ev.export_mem());
    }
}

fn run_repl(show_ast: bool, mem_enabled: bool, pretty: bool, mem_db: &Option<String>) {
    let mut ev = Evaluator::new(mem_enabled);
    if let Some(db) = mem_db {
        ev.import_mem(load_mem_file(db));
    }

    let mut buf = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        buf.clear();
        if io::stdin().read_line(&mut buf).unwrap() == 0 { break; }
        let s = buf.trim();
        if s.is_empty() { continue; }
        if s == ":q" || s == ":quit" { break; }

        match parse_expr(s) {
            Ok(ast) => {
                let r = ev.eval(&ast, None);
                let val = match r.value {
                    V::Bool(b) => serde_json::json!(b),
                    V::Unknown => serde_json::Value::Null,
                };
                let out = Out {
                    input: Some(s),
                    ast: if show_ast { Some(to_sexpr(&ast)) } else { None },
                    value: val,
                    phase: match r.phase {
                        Phase::ALIVE => "ALIVE",
                        Phase::JAM   => "JAM",
                        Phase::MEM   => "MEM",
                        Phase::VAC   => "VAC",
                    },
                };
                emit(&out, pretty);
            }
            Err(e) => eprintln!("Parse error: {e}"),
        }
    }

    if let Some(db) = mem_db {
        save_mem_file(db, &ev.export_mem());
    }
}
