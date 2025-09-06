#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhaseOp {
    Mem,
    Jam,
    Alive,
    Vac,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Bool(bool),
    Ident(String),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Imp(Box<Expr>, Box<Expr>),
    PhaseOp(PhaseOp, Box<Expr>),
}

pub fn to_sexpr(e: &Expr) -> String {
    use Expr::*;
    match e {
        Bool(true) => "true".to_string(),
        Bool(false) => "false".to_string(),
        Ident(s) => format!("id:{s}"),
        Not(x) => format!("(~ {})", to_sexpr(x)),
        And(a, b) => format!("(& {} {})", to_sexpr(a), to_sexpr(b)),
        Or(a, b) => format!("(| {} {})", to_sexpr(a), to_sexpr(b)),
        Imp(a, b) => format!("(-> {} {})", to_sexpr(a), to_sexpr(b)),
        PhaseOp(op, x) => {
            let opname = match op {
                crate::ast::PhaseOp::Mem => "@mem",
                crate::ast::PhaseOp::Jam => "@jam",
                crate::ast::PhaseOp::Alive => "@alive",
                crate::ast::PhaseOp::Vac => "@vac",
            };
            format!("({} {})", opname, to_sexpr(x))
        }
    }
}
