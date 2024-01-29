use std::ops::Index;

use crate::term;
use crate::fraction;
use crate::sign;

#[derive(Copy, Clone)]
pub(crate) enum Polynomial {
    Trinominal(Trinominal)
}

#[derive(Copy, Clone)]
pub struct Trinominal {
    pub(crate) a: fraction::Fraction,
    pub(crate) b: fraction::Fraction,
    pub(crate) c: fraction::Fraction
}

fn quadratic(a: f32, b: f32, c: f32, sign: sign::Sign) -> f32 {
    match sign {
        sign::Sign::Positive => (-b + (b * b - 4.0 * a * c)) / 2.0 * a,
        sign::Sign::Negative => (-b - (b * b - 4.0 * a * c)) / 2.0 * a,
    }
}

impl Trinominal {

    /// # Arguments
    /// None
    /// 
    /// # Returns
    /// A tuple containing the two zero values
    /// of the Trinominal
    /// 
    pub(crate) fn zero_values(&self) -> (fraction::Fraction, fraction::Fraction) {
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

    pub(crate) fn compute(&self, x: f32) -> f32 {
        let ax2     = self.a.to_float() * x * x;
        let bx      = self.b.to_float() * x;
        let c       = self.c.to_float();
        ax2 + bx + c
    }
}

impl Trinominal {
    pub(crate) fn to_string(&self) -> String {
        self.a.to_string() + " " + &self.b.to_string() + " " + &self.c.to_string()
    }
}