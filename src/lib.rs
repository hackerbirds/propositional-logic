pub mod clause;
pub mod formula;
pub mod literal;
pub mod normal_form;

#[cfg(test)]
mod tests {

    use clause::Clause;
    use formula::Formula;
    use literal::Literal;
    use normal_form::{Cnf, CnfDisjunction};

    use super::*;

    #[test]
    fn to_nnf() {
        let p = Formula::literal(1);
        let q = Formula::literal(2);

        let f = p
            .clone()
            .implies(q.clone())
            .implies(p.clone())
            .implies(p.clone());

        dbg!(f.to_nnf());
        assert_eq!(f.to_nnf(), p.not().or(q).and(p.not()).or(p))
    }

    #[test]
    fn to_cnf() {
        let p = Literal::from(1);
        let q = Literal::from(2);
        let r = Literal::from(3);
        let s = Literal::from(4);

        let not_p = p.not();
        let not_r = r.not();
        let not_s = s.not();

        let cnf = Cnf(vec![
            CnfDisjunction(vec![not_p, q]),
            CnfDisjunction(vec![not_p, not_r]),
            CnfDisjunction(vec![not_p, not_s]),
            CnfDisjunction(vec![p, q]),
        ]);

        let p: Formula = p.into();
        let q: Formula = q.into();
        let r: Formula = r.into();
        let s: Formula = s.into();

        let f = p
            .clone()
            .and(q.clone().implies(r.clone().or(s.clone())))
            .not()
            .and(p.clone().or(q.clone()));

        assert_eq!(f.to_cnf(), cnf);
    }

    #[test]
    fn is_contradiction() {
        let p = Formula::literal(1);
        let not_p = p.clone().not();

        let q = Formula::literal(2);
        let not_q = q.clone().not();

        let clause = Clause::new(vec![p.clone(), p.clone()]);
        assert!(!clause.is_contradiction());

        let clause = Clause::new(vec![p.clone(), q.clone()]);
        assert!(!clause.is_contradiction());

        let clause = Clause::new(vec![p.clone(), not_q.clone()]);
        assert!(!clause.is_contradiction());

        let clause = Clause::new(vec![q, not_q]);
        assert!(clause.is_contradiction());

        let clause = Clause::new(vec![p, not_p]);
        assert!(clause.is_contradiction());
    }
}
