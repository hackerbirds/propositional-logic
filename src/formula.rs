use std::rc::Rc;

use crate::literal::Literal;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A formula F may be:
///
/// F = p | ¬p | ¬A | A v B | A ∧ B
///
/// With p any [`Literal`], A and B are [`Formula`]s.
pub enum Formula {
    Literal(Literal),
    // A not of another formula. Does not apply to literals.
    // Literals that are "not" are stored as `Formula::Literal`.
    FormulaNot(Rc<Formula>),
    Or(Rc<Formula>, Rc<Formula>),
    And(Rc<Formula>, Rc<Formula>),
}

impl From<Literal> for Formula {
    fn from(value: Literal) -> Self {
        Formula::Literal(value)
    }
}

impl Formula {
    pub fn literal(literal: i64) -> Self {
        Formula::Literal(literal.into())
    }

    pub fn not(&self) -> Self {
        match self {
            Formula::Literal(l) => Formula::Literal(l.not()),
            _ => Formula::FormulaNot(Rc::new(self.clone())),
        }
    }

    pub fn or(&self, other: Formula) -> Self {
        Formula::Or(Rc::new(self.clone()), Rc::new(other))
    }

    pub fn and(&self, other: Formula) -> Self {
        Formula::And(Rc::new(self.clone()), Rc::new(other))
    }

    pub fn implies(&self, other: Formula) -> Self {
        // A => B is logically equivalent to ¬A v B
        self.not().or(other)
    }

    pub fn is_not_of(&self, formula: &Formula) -> bool {
        if let Formula::FormulaNot(not) = self {
            not.as_ref() == formula
        } else {
            false
        }
    }
}
