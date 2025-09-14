use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use grieg_engine::ast::{Expr, PhaseOp};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GriegParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("parse error: {0}")]
    Pest(#[from] pest::error::Error<Rule>),
}

pub fn parse_expr(input: &str) -> Result<Expr, ParseError> {
    let mut pairs = GriegParser::parse(Rule::program, input)?;
    let expr_pair = pairs.next().unwrap().into_inner().next().unwrap(); // expr
    Ok(build_any(expr_pair))
}

/* -------- descent that tolerates being handed any sub-rule ---------- */

fn build_any(pair: Pair<Rule>) -> Expr {
    match pair.as_rule() {
        Rule::expr => {
            let inner = pair.into_inner().next().unwrap();
            build_any(inner)
        }
        Rule::implies => build_implies(pair),
        Rule::or => build_or(pair),
        Rule::and => build_and(pair),
        Rule::not => build_not(pair),
        Rule::primary => build_primary(pair),
        _ => unreachable!(),
    }
}

/* ------------------ standard precedence builders ------------------- */

fn build_implies(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let left = build_or(inner.next().unwrap());
    if let Some(right) = inner.next() {
        let r = build_implies(right);
        Expr::Imp(Box::new(left), Box::new(r))
    } else {
        left
    }
}

fn build_or(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let mut node = build_and(inner.next().unwrap());
    for nxt in inner {
        node = Expr::Or(Box::new(node), Box::new(build_and(nxt)));
    }
    node
}

fn build_and(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let mut node = build_not(inner.next().unwrap());
    for nxt in inner {
        node = Expr::And(Box::new(node), Box::new(build_not(nxt)));
    }
    node
}

fn build_not(pair: Pair<Rule>) -> Expr {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    match first.as_rule() {
        Rule::not => Expr::Not(Box::new(build_not(first))),
        Rule::primary => build_primary(first),
        _ => unreachable!(),
    }
}

fn build_primary(pair: Pair<Rule>) -> Expr {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::boolean => match inner.as_str() {
            "true" | "⊤" => Expr::Bool(true),
            "false" | "⊥" => Expr::Bool(false),
            _ => unreachable!(),
        },
        Rule::ident => Expr::Ident(inner.as_str().to_string()),
        Rule::phaseop_call => {
            // phaseop "(" expr-or-implies ")"
            let mut it = inner.into_inner();
            let op = match it.next().unwrap().as_str() {
                "@mem" => PhaseOp::Mem,
                "@jam" => PhaseOp::Jam,
                "@alive" => PhaseOp::Alive,
                "@vac" => PhaseOp::Vac,
                _ => unreachable!(),
            };
            let exprish = it.next().unwrap(); // could be expr or implies (depending on grammar)
            let e = build_any(exprish);
            Expr::PhaseOp(op, Box::new(e))
        }
        Rule::expr => {
            // "(" expr ")" — descend to its child
            build_any(inner)
        }
        _ => unreachable!(),
    }
}

/* ------------------------------ tests ------------------------------ */

#[cfg(test)]
mod tests {
    use super::*;
    use grieg_engine::ast::Expr;

    #[test]
    fn smoke_implication() {
        match parse_expr("true -> false").unwrap() {
            Expr::Imp(_, _) => {}
            _ => panic!("expected Imp"),
        }
    }

    #[test]
    fn parses_parens_and_not() {
        let e = parse_expr("~false & (true | false)").unwrap();
        match e {
            Expr::And(_, _) => {}
            _ => panic!("expected And AST"),
        }
    }

    #[test]
    fn parses_parens_and_phase_call() {
        let e = parse_expr("@mem(true -> false)").unwrap();
        match e {
            Expr::PhaseOp(_, inner) => match *inner {
                Expr::Imp(_, _) => {}
                _ => panic!("phase call should contain an implication"),
            },
            _ => panic!("expected PhaseOp"),
        }
    }

    #[test]
    fn implies_is_right_assoc() {
        // a -> b -> c  ===  a -> (b -> c)
        let e = parse_expr("a -> b -> c").unwrap();
        match e {
            Expr::Imp(_, ref r) => match **r {
                Expr::Imp(_, _) => {}
                _ => panic!("expected right-associative implication (a -> (b -> c))"),
            },
            _ => panic!("expected top-level Imp"),
        }
    }

    #[test]
    fn precedence_not_and_or() {
        // ~ binds tighter than &, which binds tighter than |
        // ~a & b | c  ===  ( (~a & b) | c )
        let e = parse_expr("~a & b | c").unwrap();
        match e {
            Expr::Or(ref left, _) => match **left {
                Expr::And(ref l2, _) => match **l2 {
                    Expr::Not(_) => {}
                    _ => panic!("~ should bind tighter than &"),
                },
                _ => panic!("& should bind tighter than |"),
            },
            _ => panic!("expected top-level Or"),
        }
    }
}
