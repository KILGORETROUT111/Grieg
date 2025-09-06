//! Grieg evaluation (SpecRef: S3.*, S4.*)

use std::collections::HashMap;

use crate::ast::{Expr, PhaseOp};
use crate::phase::Phase;
use crate::value::V;

/// Result of evaluating an expression.
#[derive(Clone, Debug)]
pub struct EvalResult {
    pub value: V,
    pub phase: Phase,
}

/// Evaluator with optional MEM semantics and a tiny persistent store.
/// NOTE: `mem_store` is ONLY for persistence (--mem-db). We don't write to it during eval yet.
pub struct Evaluator {
    /// Enable MEM semantics (SpecRef: S3.10).
    pub mem_enabled: bool,
    /// Persistent MEM facts (identifier -> bool). Used by CLI load/save.
    mem_store: HashMap<String, bool>,
}

impl Evaluator {
    /// Create a new evaluator.
    pub fn new(mem_enabled: bool) -> Self {
        Self {
            mem_enabled,
            mem_store: HashMap::new(),
        }
    }

    // ----------------------------------------------------------------
    // Persistence hooks (used by grieg-cli --mem-db).
    // These do not change eval rules; they only make free idents resolvable.
    // ----------------------------------------------------------------

    /// SpecRef: S3.10 — import external MEM facts for persistence.
    pub fn import_mem(&mut self, map: HashMap<String, bool>) {
        if !self.mem_enabled {
            return;
        }
        self.mem_store.extend(map);
    }

    /// SpecRef: S3.10 — export current MEM facts for persistence.
    pub fn export_mem(&self) -> HashMap<String, bool> {
        if !self.mem_enabled {
            return HashMap::new();
        }
        self.mem_store.clone()
    }

    // ----------------------------------------------------------------
    // Evaluation (big-step)
    // ----------------------------------------------------------------

    /// Evaluate an expression. Second arg kept to match existing call sites.
    pub fn eval(&mut self, e: &Expr, _unused: Option<&mut ()>) -> EvalResult {
        match e {
            Expr::Bool(b) => EvalResult {
                value: V::Bool(*b),
                phase: Phase::ALIVE, // S3.1
            },

            Expr::Ident(name) => {
                // S3.2: free identifiers → Unknown,VAC unless a MEM fact exists
                if let Some(b) = self.mem_store.get(name) {
                    EvalResult {
                        value: V::Bool(*b),
                        phase: Phase::MEM,
                    }
                } else {
                    EvalResult {
                        value: V::Unknown,
                        phase: Phase::VAC,
                    }
                }
            }

            Expr::Not(x) => {
                let r = self.eval(x, None);
                let v = match r.value.to_bool() {
                    Some(true) => V::Bool(false),
                    Some(false) => V::Bool(true),
                    None => V::Unknown,
                };
                let phase = if v.to_bool().is_none() {
                    Phase::VAC
                } else {
                    r.phase
                }; // S4.6
                EvalResult { value: v, phase }
            }

            Expr::And(a, b) => {
                let ra = self.eval(a, None);
                let rb = self.eval(b, None);
                let v = and3(ra.value, rb.value); // S3.4
                let mut phase = join(ra.phase, rb.phase); // S4.3
                if v.to_bool().is_none() {
                    phase = Phase::VAC;
                } // S4.6
                EvalResult { value: v, phase }
            }

            Expr::Or(a, b) => {
                let ra = self.eval(a, None);
                let rb = self.eval(b, None);
                let v = or3(ra.value, rb.value); // S3.5
                let mut phase = join(ra.phase, rb.phase);
                if v.to_bool().is_none() {
                    phase = Phase::VAC;
                }
                EvalResult { value: v, phase }
            }

            Expr::Imp(a, b) => {
                let ra = self.eval(a, None);
                let rb = self.eval(b, None);
                let v = imp3(ra.value, rb.value); // S3.6
                let mut phase = join(ra.phase, rb.phase);
                if v.to_bool().is_none() {
                    phase = Phase::VAC;
                }
                EvalResult { value: v, phase }
            }

            Expr::PhaseOp(op, x) => {
                let r = self.eval(x, None);
                match op {
                    PhaseOp::Alive => EvalResult {
                        value: r.value,
                        phase: Phase::ALIVE,
                    }, // S3.7, S4.4
                    PhaseOp::Jam => EvalResult {
                        value: r.value,
                        phase: Phase::JAM,
                    }, // S3.8, S4.4
                    PhaseOp::Vac => EvalResult {
                        value: V::Unknown,
                        phase: Phase::VAC,
                    }, // S3.9
                    PhaseOp::Mem => EvalResult {
                        value: r.value,
                        phase: Phase::MEM,
                    }, // S3.10
                }
            }
        }
    }
}

// ----------------------------------------------------------------
// Truth-functions with Unknown lifting (S3.3–S3.6)
// ----------------------------------------------------------------

fn and3(a: V, b: V) -> V {
    match (a.to_bool(), b.to_bool()) {
        (Some(false), _) | (_, Some(false)) => V::Bool(false),
        (Some(true), Some(true)) => V::Bool(true),
        _ => V::Unknown,
    }
}

fn or3(a: V, b: V) -> V {
    match (a.to_bool(), b.to_bool()) {
        (Some(true), _) | (_, Some(true)) => V::Bool(true),
        (Some(false), Some(false)) => V::Bool(false),
        _ => V::Unknown,
    }
}

fn imp3(a: V, b: V) -> V {
    match (a.to_bool(), b.to_bool()) {
        (Some(false), _) => V::Bool(true),
        (Some(true), Some(v)) => V::Bool(v),
        (Some(true), None) => V::Unknown,
        (None, Some(true)) => V::Bool(true),
        _ => V::Unknown,
    }
}

// ----------------------------------------------------------------
// Phase join (S4.*)
// ----------------------------------------------------------------

fn join(a: Phase, b: Phase) -> Phase {
    use Phase::*;
    if a == JAM || b == JAM {
        return JAM; // S4.1 JAM dominance
    }
    if (a == MEM && b == ALIVE) || (a == ALIVE && b == MEM) || (a == MEM && b == MEM) {
        return MEM; // S4.2 MEM dominates ALIVE, idempotent
    }
    if (a == VAC && b == ALIVE) || (a == ALIVE && b == VAC) {
        return ALIVE;
    }
    if a == VAC && b == VAC {
        return VAC;
    }
    ALIVE
}
