use std::arch::x86_64::_MM_FLUSH_ZERO_MASK;

use crate::{fraction::ExtendedReal, polynomial::{self, Polynomial}, sign::Sign};

/// ## Mathematical range operator
/// ### `Inclusive`
///     Either `[` or `]`
/// ### `Exclusive`
///     Either `(` or `)`
pub(crate) enum RangeOperator {
    Inclusive,
    Exclusive
}

/// ## Domain
/// ### `range_ops`
/// The range operators on either side of 
/// the domain definition.
/// ### `values`
/// The highest and lowest value in the
/// domain respectively.
pub(crate) struct Domain {
    range_ops: (RangeOperator, RangeOperator),
    values: (ExtendedReal, ExtendedReal)
}

impl Domain {
    pub(crate) fn to_string(&self) -> String {

        // Left range operator
        let l = match self.range_ops.0 {
            RangeOperator::Inclusive => "[",
            RangeOperator::Exclusive => "(",
        };

        // Right range operator
        let r = match self.range_ops.1 {
            RangeOperator::Inclusive => "]",
            RangeOperator::Exclusive => ")",
        };

        // Lowest value
        let lval = self.values.0.to_string();
        // Highest value
        let rval = self.values.1.to_string();

        l.to_owned() + &lval + ", " + &rval + r
    }
}


pub(crate) fn dom(f: polynomial::Polynomial) -> Domain {
    match f {
        polynomial::Polynomial::Trinominal(_) => Domain { // The domain of a trinominal is always (-∞, +∞)
            range_ops: (RangeOperator::Exclusive, RangeOperator::Exclusive),
            values: (ExtendedReal::Infinity(Sign::Negative), ExtendedReal::Infinity(Sign::Positive)),
        },
    }
}

/// ### Arguments
/// f: The polynomial to be examined
/// 
/// ### Returns
/// (bool, bool) for Reflection Symmetry and  Rotational
/// Symmetry of 180 degrees respectively
/// 
/// ### Note: 
/// this does not "prove" symmetry, but merely
/// estimates wether or not a given function
/// is symmetric or not.





pub(crate) fn sym(f: polynomial::Polynomial) -> (bool, bool) {

    let reflection_symmetry = match f {
        Polynomial::Trinominal(f) => {
            (0..9999).all(|i|
                f.compute(i as f32) == f.compute(-i as f32)
            )
        },
    };

    let rotational_symmetry = match f {
        Polynomial::Trinominal(f) => {
            (0..9999).all(|i|
                f.compute(i as f32) == -f.compute(i as f32)
            )
        },
    };

    (reflection_symmetry, rotational_symmetry)
}