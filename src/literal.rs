use std::{fmt::Debug, num::NonZeroI64};

/// A literal is a propositional value that may be true or false.
///
/// It is represented and distinguished from other literals by its
/// non-zero [`i64`] id.
/// 
/// The [`Literal`] associated with id `-n` is treated as the logical
/// opposite of the [`Literal`] with id `n`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Literal(NonZeroI64 /* id */);

impl Literal {
    pub const fn not(&self) -> Literal {
        Literal(self.0.wrapping_neg())
    }

    pub fn is_not(&self) -> bool {
        self.0.is_negative()
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Literal(
            NonZeroI64::new(value)
                .expect("0 is not a valid literal id. Please use a non-zero integer"),
        )
    }
}
