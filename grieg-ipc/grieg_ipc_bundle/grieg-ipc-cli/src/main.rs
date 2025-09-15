use std::env;
use std::io::{self, Write};

use chrono::Utc;
use grieg_engine::eval::Evaluator;
use grieg_engine::phase::Phase;
use grieg_engine::value::V;
use grieg_parser::parse_expr;
use grieg_telemetry::{JsonlSink, TelemetryEvent, TelemetrySink};

fn print_usage() {
    eprintln!(r#"Usage:
  grieg-ipc-cli --expr "<expr>" [--mem] [--ast] [--pretty] [--jsonl <file>]
  grieg-ipc-cli --jsonl <file> --repl [--mem] [--ast] [--pretty]

Examples:
  grieg-ipc-cli --expr "@mem(true -> false)" --mem --pretty --jsonl ./ipc.jsonl
  grieg-ipc-cli --repl --mem --jsonl ./ipc.jsonl
"#);
}

fn v_to_bool(v: &V) -> Option<bool> { v.to_bool() }

fn phase_to_str(p: &Phase) -> &'static str {
    match p {
        Phase::ALIVE => "ALIVE",
        Phase::JAM => "JAM",
        Phase::MEM => "MEM",
        Phase::VAC => "VAC",
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let mut expr_arg: Option<String> = None;
    let mut repl = false;
    let mut mem = false;
    let mut ast = false;
    let mut pretty = false;
    let mut jsonl_path: Option<String> = None;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--expr" => expr_arg = args.next(),
            "--repl" => repl = true,
            "--mem" => mem = true,
            "--ast" => ast = true,
            "--pretty" => pretty = true,
            "--jsonl" => jsonl_path = args.next(),
            _ => {
                eprintln!("Unknown arg: {}", a);
                print_usage();
                std::process::exit(2);
            }
        }
    }

    if expr_arg.is_none() && !repl {
        print_usage();
        std::process::exit(2);
    }

    let mut sink_opt: Option<JsonlSink> = match jsonl_path {
        Some(p) => Some(JsonlSink::create(p)?),
        None => None,
    };

    let mut ev = Evaluator::new(mem);

    if let Some(s) = expr_arg {
        run_once(&mut ev, &s, ast, pretty, sink_opt.as_mut())?;
        if let Some(sink) = sink_opt.as_mut() { sink.flush()?; }
        return Ok(());
    }

    if repl {
        let stdin = io::stdin();
        let mut input = String::new();
        loop {
            input.clear();
            eprint!("> ");
            io::stderr().flush().ok();
            if stdin.read_line(&mut input)? == 0 { break; }
            let line = input.trim();
            if line == ":q" || line == ":quit" { break; }
            if !line.is_empty() {
                if let Err(e) = run_once(&mut ev, line, ast, pretty, sink_opt.as_mut()) {
                    eprintln!("error: {}", e);
                }
            }
        }
        if let Some(sink) = sink_opt.as_mut() { sink.flush()?; }
        return Ok(());
    }

    Ok(())
}

fn run_once(
    ev: &mut Evaluator,
    input: &str,
    show_ast: bool,
    pretty: bool,
    sink_opt: Option<&mut JsonlSink>,
) -> Result<(), Box<dyn std::error::Error>> {
    let expr = parse_expr(input)?;
    let ast_s = grieg_engine::ast::to_sexpr(&expr);
    let res = ev.eval(&expr, None);

    if pretty {
        println!("Input: {}", input);
        if show_ast { println!("AST:   {}", ast_s); }
        match v_to_bool(&res.value) {
            Some(b) => println!("Value: {}", b),
            None => println!("Value: null"),
        }
        println!("Phase: {}", phase_to_str(&res.phase));
    } else {
        let json = serde_json::json!({
            "input": input,
            "ast": if show_ast { serde_json::Value::String(ast_s.clone()) } else { serde_json::Value::Null },
            "value": v_to_bool(&res.value),
            "phase": phase_to_str(&res.phase),
        });
        println!("{}", serde_json::to_string(&json)?);
    }

    if let Some(sink) = sink_opt {
        let mut tev = TelemetryEvent::new();
        tev.ts = Utc::now();
        tev.expr = Some(input.to_string());
        if show_ast { tev.ast = Some(ast_s); }
        tev.phase = Some(phase_to_str(&res.phase).to_string());
        tev.value_bool = v_to_bool(&res.value);
        tev.value_text = Some(format!("{:?}", res.value));
        tev.jam = matches!(res.phase, Phase::JAM);
        tev.sink = false;
        sink.record(&tev)?;
    }

    Ok(())
}
