use std::env;
use std::fs::File;
use std::io::{self, BufRead, Read};

use serde::Deserialize;
use serde_json::{json, Value};

use grieg_engine::{Evaluator};
use grieg_engine::phase::Phase;
use grieg_engine::value::V;
use grieg_parser::parse_expr;

/// JSONL input schema for conformance runs.
#[derive(Debug, Deserialize)]
struct JsonlCase {
    expr: String,
    #[serde(default)]
    mem: Option<bool>,
    #[serde(default)]
    expect_phase: Option<String>,
    #[serde(default)]
    note: Option<String>,
}

#[derive(serde::Serialize)]
struct Out<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ast: Option<String>,
    value: Value,
    phase: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expect_phase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

fn phase_to_str(p: &Phase) -> &'static str {
    match p {
        Phase::JAM => "JAM",
        Phase::MEM => "MEM",
        Phase::VAC => "VAC",
        Phase::ALIVE => "ALIVE",
    }
}

fn value_to_json(v: &V) -> Value {
    match v {
        V::Bool(b) => json!(*b),
        V::Unknown => Value::Null,
    }
}

fn to_ast_string(ast: &impl std::fmt::Debug) -> String {
    // Use Debug for a stable developer-readable AST print (matches earlier CLI behavior).
    format!("{ast:?}")
}

fn emit(out: &Out, pretty: bool) {
    if pretty {
        println!("{}", serde_json::to_string_pretty(out).unwrap());
    } else {
        println!("{}", serde_json::to_string(out).unwrap());
    }
}

fn handle_jsonl(path: &str, global_mem: bool, want_ast: bool, pretty: bool) -> io::Result<()> {
    let f = File::open(path)?;
    let r = io::BufReader::new(f);

    for (lineno, line_res) in r.lines().enumerate() {
        let lineno = lineno + 1;
        let line = match line_res {
            Ok(s) => s,
            Err(e) => {
                println!("{}", json!({ "line": lineno, "error": format!("io: {e}") }));
                continue;
            }
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Try to parse JSONL case first.
        if let Ok(tc) = serde_json::from_str::<JsonlCase>(line) {
            // Parse Grieg expression
            match parse_expr(&tc.expr) {
                Ok(ast) => {
                    let mem_enabled = tc.mem.unwrap_or(global_mem);
                    let mut ev = Evaluator::new(mem_enabled);
                    let res = ev.eval(&ast, None);

                    let phase_str = phase_to_str(&res.phase);
                    let ok = tc
                        .expect_phase
                        .as_ref()
                        .map(|exp| exp.as_str() == phase_str);

                    let out = Out {
                        input: Some(&tc.expr),
                        ast: if want_ast { Some(to_ast_string(&ast)) } else { None },
                        value: value_to_json(&res.value),
                        phase: phase_str,
                        ok,
                        expect_phase: tc.expect_phase,
                        note: tc.note,
                    };
                    emit(&out, pretty);
                }
                Err(e) => {
                    println!("{}", json!({
                        "line": lineno,
                        "input": tc.expr,
                        "error": format!("parse: {e}")
                    }));
                }
            }
        } else {
            // Fallback: treat line as a raw Grieg expression (legacy batch mode).
            match parse_expr(line) {
                Ok(ast) => {
                    let mut ev = Evaluator::new(global_mem);
                    let res = ev.eval(&ast, None);
                    let out = Out {
                        input: Some(line),
                        ast: if want_ast { Some(to_ast_string(&ast)) } else { None },
                        value: value_to_json(&res.value),
                        phase: phase_to_str(&res.phase),
                        ok: None,
                        expect_phase: None,
                        note: None,
                    };
                    emit(&out, pretty);
                }
                Err(e) => {
                    println!("{}", json!({
                        "line": lineno,
                        "error": format!("parse: {e}")
                    }));
                }
            }
        }
    }

    Ok(())
}

fn eval_single(expr: &str, mem: bool, want_ast: bool, pretty: bool) {
    match parse_expr(expr) {
        Ok(ast) => {
            let mut ev = Evaluator::new(mem);
            let res = ev.eval(&ast, None);
            let out = Out {
                input: Some(expr),
                ast: if want_ast { Some(to_ast_string(&ast)) } else { None },
                value: value_to_json(&res.value),
                phase: phase_to_str(&res.phase),
                ok: None,
                expect_phase: None,
                note: None,
            };
            emit(&out, pretty);
        }
        Err(e) => {
            eprintln!("Parse error: {e}");
        }
    }
}

fn print_help() {
    eprintln!(
        "\
Grieg CLI

USAGE:
  grieg-cli --expr '<EXPR>' [--mem] [--ast] [--pretty]
  grieg-cli --jsonl <FILE> [--mem] [--ast] [--pretty]
  grieg-cli --help

FLAGS:
  --expr <EXPR>     Evaluate a single Grieg expression
  --jsonl <FILE>    Evaluate a JSONL file with objects: {{expr, mem?, expect_phase?, note?}}
  --mem             Enable MEM transport
  --ast             Include AST in output
  --pretty          Pretty-print JSON output
  --help            Show this help
"
    );
}

fn main() {
    // Minimal arg parsing (no extra deps).
    let mut args = env::args().skip(1);
    let mut expr_opt: Option<String> = None;
    let mut jsonl_opt: Option<String> = None;
    let mut mem_flag = false;
    let mut ast_flag = false;
    let mut pretty_flag = false;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--expr" => {
                if let Some(e) = args.next() {
                    expr_opt = Some(e);
                } else {
                    eprintln!("--expr requires a value");
                    std::process::exit(2);
                }
            }
            "--jsonl" => {
                if let Some(p) = args.next() {
                    jsonl_opt = Some(p);
                } else {
                    eprintln!("--jsonl requires a file path");
                    std::process::exit(2);
                }
            }
            "--mem" => mem_flag = true,
            "--ast" => ast_flag = true,
            "--pretty" => pretty_flag = true,
            "--help" | "-h" => {
                print_help();
                return;
            }
            other => {
                eprintln!("Unknown argument: {other}");
                print_help();
                std::process::exit(2);
            }
        }
    }

    if let Some(jsonl_path) = jsonl_opt {
        if let Err(e) = handle_jsonl(&jsonl_path, mem_flag, ast_flag, pretty_flag) {
            eprintln!("cannot open file: {e}");
            std::process::exit(1);
        }
        return;
    }

    if let Some(expr) = expr_opt {
        eval_single(&expr, mem_flag, ast_flag, pretty_flag);
        return;
    }

    // If no --expr/--jsonl, read from stdin (each line an expression).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() && !input.trim().is_empty() {
        for (lineno, line) in input.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match parse_expr(line) {
                Ok(ast) => {
                    let mut ev = Evaluator::new(mem_flag);
                    let res = ev.eval(&ast, None);
                    let out = Out {
                        input: Some(line),
                        ast: if ast_flag { Some(to_ast_string(&ast)) } else { None },
                        value: value_to_json(&res.value),
                        phase: phase_to_str(&res.phase),
                        ok: None,
                        expect_phase: None,
                        note: None,
                    };
                    emit(&out, pretty_flag);
                }
                Err(e) => {
                    println!("{}", json!({
                        "line": lineno + 1,
                        "error": format!("parse: {e}")
                    }));
                }
            }
        }
        return;
    }

    print_help();
}
