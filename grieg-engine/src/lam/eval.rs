use super::ast::L;

pub struct LamResult {
    pub term: L,
    pub steps: usize,
    pub diverged: bool,
}

/// Leftmost-outermost β-reduction skeleton with fuel (TODO: implement)
pub fn eval_normal_order(t: L, _fuel: usize) -> LamResult {
    LamResult { term: t, steps: 0, diverged: false }
}
