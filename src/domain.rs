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
    pub(crate) range_ops: (RangeOperator, RangeOperator),
    pub(crate) values: (ExtendedReal, ExtendedReal)
}

impl Domain {
    fn to_string(&self) -> String {

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

impl std::fmt::Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}