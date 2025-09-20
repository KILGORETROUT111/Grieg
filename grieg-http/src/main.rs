use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use std::env;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

#[derive(Deserialize)]
struct EvalIn {
    prompt: String,
    mem: Option<bool>,
    ast: Option<bool>,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/v1/evaluate", post(evaluate));

    // Bind with a tuple to avoid type-annotation issues
    let listener = tokio::net::TcpListener::bind(("127.0.0.1", 8000))
        .await
        .expect("bind 127.0.0.1:8000");
    println!("grieg-http listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn evaluate(Json(inp): Json<EvalIn>) -> Json<Value> {
    match run_cli(&inp.prompt).await {
        Ok(v) => Json(v),
        Err(e) => Json(json!({ "rc": 1, "error": e })),
    }
}

/// Calls the real Rust CLI. You can override with env vars:
///   GRIEG_CLI_CMD  (default: "grieg-cli")
///   GRIEG_CLI_ARGS (default: empty; space-separated)
async fn run_cli(prompt: &str) -> Result<Value, String> {
    let cmd = env::var("GRIEG_CLI_CMD").unwrap_or_else(|_| "grieg-cli".into());
    let extra = env::var("GRIEG_CLI_ARGS").unwrap_or_default();
    let extra_parts: Vec<&str> =
        if extra.trim().is_empty() { vec![] } else { extra.split_whitespace().collect() };

    // Try common layouts; adjust later if your CLI differs
    let layouts: Vec<Vec<String>> = vec![
        vec!["eval".into(), "--json".into(), prompt.into()],
        vec!["--json".into(), "eval".into(), prompt.into()],
        vec!["eval".into(), prompt.into()],
        vec![prompt.into()],
    ];

    for args in layouts {
        let mut full: Vec<&str> = extra_parts.clone();
        for s in &args { full.push(s.as_str()); }
        if let Ok(out) = capture(&cmd, &full, None).await {
            if !out.status.success() { continue; }
            if let Ok(js) = serde_json::from_slice::<Value>(&out.stdout) { return Ok(js); }
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            return Ok(json!({"rc":0,"text":text}));
        }
    }

    // stdin fallback (no args; prompt via stdin)
    let out = capture(&cmd, &extra_parts, Some(prompt)).await?;
    if !out.status.success() {
        return Err(format!(
            "cli exit {}: {}",
            out.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    if let Ok(js) = serde_json::from_slice::<Value>(&out.stdout) { Ok(js) } else {
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        Ok(json!({"rc":0,"text":text}))
    }
}

struct Capt { status: std::process::ExitStatus, stdout: Vec<u8>, stderr: Vec<u8> }

async fn capture(cmd: &str, args: &[&str], stdin_text: Option<&str>) -> Result<Capt, String> {
    let mut c = Command::new(cmd);
    c.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    if stdin_text.is_some() { c.stdin(Stdio::piped()); }
    let mut child = c.spawn().map_err(|e| format!("spawn {cmd}: {e}"))?;
    if let Some(text) = stdin_text {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).await.map_err(|e| e.to_string())?;
        }
    }
    let out = child.wait_with_output().await.map_err(|e| e.to_string())?;
    Ok(Capt{ status: out.status, stdout: out.stdout, stderr: out.stderr })
}
