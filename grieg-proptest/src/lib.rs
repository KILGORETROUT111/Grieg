#[cfg(test)]
mod props {
    use grieg_engine::eval::Evaluator;
    use grieg_engine::phase::Phase;
    use grieg_parser::parse_expr;

    #[test]
    fn implication_truth_table() {
        let mut ev = Evaluator::new(false);

        let e = parse_expr("true -> false").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.value.to_bool(), Some(false));
        assert_eq!(r.phase, Phase::ALIVE);

        let e = parse_expr("true -> true").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.value.to_bool(), Some(true));
        assert_eq!(r.phase, Phase::ALIVE);

        let e = parse_expr("false -> true").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.value.to_bool(), Some(true));
        assert_eq!(r.phase, Phase::ALIVE);

        let e = parse_expr("false -> false").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.value.to_bool(), Some(true));
        assert_eq!(r.phase, Phase::ALIVE);
    }

    #[test]
    fn mem_recovery_simple() {
        let mut ev = Evaluator::new(true);
        let e = parse_expr("@mem(true -> false)").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.value.to_bool(), Some(false));
        assert_eq!(r.phase, Phase::MEM);
    }

    #[test]
    fn vac_for_free_ident() {
        let mut ev = Evaluator::new(false);
        let e = parse_expr("@vac(x)").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.phase, Phase::VAC);
        assert_eq!(r.value.to_bool(), None);
    }

    #[test]
    fn jam_dominance_in_join() {
        let mut ev = Evaluator::new(false);
        let e = parse_expr("@jam(true & true)").unwrap();
        let r = ev.eval(&e, None);
        assert_eq!(r.phase, Phase::JAM);
        assert_eq!(r.value.to_bool(), Some(true));
    }
}


#[test]
fn mem_transport_preserves_truth() {
    use grieg_engine::{eval::Evaluator, value::V};
    use grieg_parser::parse_expr;

    let mut ev = Evaluator::new(true); // mem enabled
    for s in &["true", "false", "~false & (true | false)"] {
        let e = parse_expr(s).unwrap();
        let r0 = ev.eval(&e, None);
        let e_mem = parse_expr(&format!("@mem({})", s)).unwrap();
        let r1 = ev.eval(&e_mem, None);
        assert_eq!(r0.value.to_bool(), r1.value.to_bool(), "MEM must preserve boolean truth");
    }
}

#[test]
fn vac_projects_to_counterfactual() {
    use grieg_engine::{eval::Evaluator, phase::Phase};
    use grieg_parser::parse_expr;

    let mut ev = Evaluator::new(true);
    let e = parse_expr("@vac(x)").unwrap();
    let r = ev.eval(&e, None);
    assert!(r.value.to_bool().is_none(), "VAC => no total boolean");
    assert_eq!(r.phase, Phase::VAC, "VAC phase marks counterfactual sheet");
}

