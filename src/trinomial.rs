use std::ops::Index;
use crate::domain::Domain;
use crate::domain::RangeOperator;
use crate::fraction::fraction_from_float;
use crate::fraction::ExtendedReal;
use crate::fraction::Fraction;
use crate::polynomial::Polynomial;
use crate::sign::Sign;
use crate::term;
use crate::fraction;
use crate::sign;

#[derive(Copy, Clone)]
pub struct Trinomial {
    pub(crate) a: fraction::Fraction,
    pub(crate) b: fraction::Fraction,
    pub(crate) c: fraction::Fraction
}

fn quadratic(a: f32, b: f32, c: f32, sign: Sign) -> f32 {
    match sign {
        Sign::Positive => (-b + (b * b - 4.0 * a * c)) / 2.0 * a,
        Sign::Negative => (-b - (b * b - 4.0 * a * c)) / 2.0 * a,
    }
}

impl Polynomial for Trinomial {

    fn zero_values(&self) -> (fraction::Fraction, fraction::Fraction) {
        /*
         * (-b ± √(b² - 4ac)) / 2a
         */
        let a = self.a.to_float();
        let b = self.b.to_float();
        let c = self.c.to_float();

        let x1 = quadratic(a, b, c, sign::Sign::Positive);
        let x2 = quadratic(a, b, c, sign::Sign::Negative);

        (fraction::fraction_from_float(x1), fraction::fraction_from_float(x2))
    }

    fn compute(&self, x: Fraction) -> f32 {
        let _x = x.to_float();

        let ax2     = self.a.to_float() * _x * _x;
        let bx      = self.b.to_float() * _x;
        let c       = self.c.to_float();

        ax2 + bx + c
    }

    fn domain(&self) -> Domain {
        Domain {
            range_ops: (RangeOperator::Exclusive, RangeOperator::Exclusive),
            values: (ExtendedReal::Infinity(Sign::Negative), ExtendedReal::Infinity(Sign::Positive)),
        }
    }

    fn to_string(&self) -> String {
        self.a.to_string() + " " + &self.b.to_string() + " " + &self.c.to_string()
    }

    fn symmetry(&self) -> (bool, bool) {

        let reflection_symmetry = (0..9999).all(|i|
            self.compute(fraction_from_float(i as f32)) == self.compute(fraction_from_float(-i as f32))
        );

        let rotational_symmetry = (0..9999).all(|i|
            self.compute(fraction_from_float(i as f32)) == -self.compute(fraction_from_float(i as f32))
        );
    
        (reflection_symmetry, rotational_symmetry)
    }
}
