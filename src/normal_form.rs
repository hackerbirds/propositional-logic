use crate::{formula::Formula, literal::Literal};

/// A set of disjunctions of [`Literal`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CnfDisjunction(pub(crate) Vec<Literal>);

/// A conjunctive normal form is a conjunction of disjunctions.
///
/// In our case, it is a set of conjunctions of [`CnfDisjunction`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cnf(pub(crate) Vec<CnfDisjunction>);

impl Formula {
    /// Converts a formula into negation normal form by recursively
    /// applying rules.
    pub fn to_nnf(&self) -> Formula {
        match self {
            Formula::FormulaNot(formula_not) => match formula_not.as_ref() {
                Formula::FormulaNot(p) => p.clone().to_nnf(),
                Formula::Or(p, q) => Formula::And(p.not().to_nnf().into(), q.not().to_nnf().into()),
                Formula::And(p, q) => Formula::Or(p.not().to_nnf().into(), q.not().to_nnf().into()),
                _ => Formula::FormulaNot(formula_not.clone().into()),
            },
            Formula::And(a, b) => Formula::And(a.to_nnf().into(), b.to_nnf().into()),
            Formula::Or(a, b) => Formula::Or(a.to_nnf().into(), b.to_nnf().into()),
            Formula::Literal(p) => Formula::Literal(*p),
        }
    }

    /// Recursively converts an NNF into a CNF.
    ///
    /// This function is only valid is
    pub fn to_cnf(&self) -> Cnf {
        let nnf = self.to_nnf();
        let cnf = nnf.cnf_rule();

        let mut literals = Vec::new();
        cnf.separate_and(&mut literals);

        Cnf(literals)
    }

    /// Separate a [`Formula`]s nested ANDs and ORs into a [`Cnf`]
    /// (which is a [`Vec<CnfDisjunction>`])
    fn separate_and(&self, literals: &mut Vec<CnfDisjunction>) {
        match self {
            Formula::Or(rc, rc1) => {
                let mut or_literals = Vec::new();
                rc.separate_or(&mut or_literals);
                rc1.separate_or(&mut or_literals);
                let cnf_literal = CnfDisjunction(or_literals);
                literals.push(cnf_literal);
            }
            Formula::And(rc, rc1) => {
                rc.separate_and(literals);
                rc1.separate_and(literals);
            }
            _ => unreachable!("Current formula form only has ANDs or ORs"),
        }
    }

    /// Separate a [`Formula`]s nested ORs into [`CnfDisjunction`]
    /// (which is a [`Vec<Literal>`])
    fn separate_or(&self, or_literals: &mut Vec<Literal>) {
        match self {
            Formula::Literal(literal) => {
                or_literals.push(*literal);
            }
            Formula::Or(f_p, f_q) => {
                f_p.separate_or(or_literals);
                f_q.separate_or(or_literals);
            }
            _ => unreachable!("Current formula should only contain Or or Literal"),
        }
    }

    /// Recursively applies CNF conversion rules to [`Formula`]
    fn cnf_rule(&self) -> Formula {
        match self {
            Formula::Or(a, b) => {
                if let Formula::And(p, q) = a.as_ref() {
                    Formula::And(
                        Formula::Or(p.clone(), b.clone()).cnf_rule().into(),
                        Formula::Or(q.clone(), b.clone()).cnf_rule().into(),
                    )
                    .cnf_rule()
                } else if let Formula::And(q, r) = b.as_ref() {
                    Formula::And(
                        Formula::Or(a.clone(), q.clone()).cnf_rule().into(),
                        Formula::Or(a.clone(), r.clone()).cnf_rule().into(),
                    )
                } else {
                    // stop recursion
                    Formula::Or(a.clone(), b.clone())
                }
            }
            Formula::And(a, b) => {
                // continue recursion
                Formula::And(a.cnf_rule().into(), b.cnf_rule().into())
            }
            other => other.clone(),
        }
    }
}
