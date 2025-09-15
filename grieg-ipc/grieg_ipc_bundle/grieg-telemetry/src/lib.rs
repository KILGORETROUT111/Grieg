use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Minimal event schema for the "invariant pulse channel" (v0.1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    /// ISO8601 timestamp (UTC).
    pub ts: DateTime<Utc>,
    /// Original expression text, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expr: Option<String>,
    /// S-expression AST string, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ast: Option<String>,
    /// Phase where the evaluation ended (ALIVE, JAM, MEM, VAC).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// A conservative textual rendering of the value (engine-independent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_text: Option<String>,
    /// If the engine exposes a boolean view, capture it here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_bool: Option<bool>,

    /// Whether a fixed-point / sink was reached (if applicable).
    #[serde(default)]
    pub sink: bool,
    /// Whether a boundary jam was encountered.
    #[serde(default)]
    pub jam: bool,

    /// Optional amplitude / channel metadata (future use).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

impl TelemetryEvent {
    pub fn new() -> Self {
        Self {
            ts: Utc::now(),
            expr: None,
            ast: None,
            phase: None,
            value_text: None,
            value_bool: None,
            sink: false,
            jam: false,
            amplitude: None,
            channel: Some("ipc.v0".to_string()),
        }
    }
}

/// A simple trait for sinks that can record telemetry events.
pub trait TelemetrySink {
    fn record(&mut self, ev: &TelemetryEvent) -> std::io::Result<()>;
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

/// JSON Lines sink: one JSON object per line.
pub struct JsonlSink {
    writer: BufWriter<File>,
}

impl JsonlSink {
    pub fn create<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let f = File::create(path)?;
        Ok(Self { writer: BufWriter::new(f) })
    }
}

impl TelemetrySink for JsonlSink {
    fn record(&mut self, ev: &TelemetryEvent) -> std::io::Result<()> {
        let line = serde_json::to_string(ev)?;
        self.writer.write_all(line.as_bytes())?;
        self.writer.write_all(b"\n")?;
        Ok(())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }
}
