use crate::formula::Formula;

pub struct Clause(Vec<Formula>);

impl Clause {
    pub fn new(vec: Vec<Formula>) -> Self {
        Self(vec)
    }

    /// Naive O(n^2) check to see if a Clause
    /// constains a propositional variable p
    /// and it's opposite ¬p
    pub fn is_contradiction(&self) -> bool {
        for formula_a in &self.0 {
            for formula_b in &self.0 {
                if formula_b.is_not_of(formula_a) {
                    return true;
                }
            }
        }

        false
    }
}
