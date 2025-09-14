pub mod eval;
pub mod phase;
pub mod ast;
pub mod value;

#[cfg(feature = "emit_geometry")]
pub mod trace;

// Public facade (stable API)
pub use crate::eval::Evaluator;
pub use crate::phase::Phase;          // if you have a Phase enum module
pub use crate::ast::Expr;             // if Expr lives in engine::ast and is useful to expose