#[cfg(test)]
mod props {
    use grieg_engine::ast::Expr::*;
    use grieg_engine::{ast::Expr, eval, eval::Evaluator, phase::Phase, value::V};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn implication_truth_table(p in any::<bool>(), q in any::<bool>()) {
            let e = Imp(Box::new(Bool(p)), Box::new(Bool(q)));
            let r = eval::eval(&e, None);
            prop_assert_eq!(r.value, V::Bool((!p) || q));
            prop_assert_eq!(r.phase, Phase::ALIVE);
        }
    }

    #[test]
    fn vac_for_free_ident() {
        let e = Expr::Ident("x".to_string());
        let r = eval::eval(&e, None);
        assert!(matches!(r.value, V::Unknown));
        assert_eq!(r.phase, Phase::VAC);
    }

    #[test]
    fn mem_recovery_simple() {
        let mut ev = Evaluator::new(true);
        let e = Expr::PhaseOp(grieg_engine::ast::PhaseOp::Mem, Box::new(Expr::Bool(true)));
        let r1 = ev.eval(&e, None);
        let r2 = ev.eval(&e, None);
        assert_eq!(r1.value, r2.value);
        assert_eq!(r2.phase, Phase::MEM);
    }

    #[test]
    fn jam_dominance_in_join() {
        use grieg_engine::phase::Phase::*;
        assert_eq!(JAM.join(ALIVE), JAM);
        assert_eq!(JAM.join(MEM), JAM);
        assert_eq!(JAM.join(VAC), JAM);
        assert_eq!(MEM.join(VAC), MEM);
        assert_eq!(VAC.join(ALIVE), VAC);
    }
}
