use crate::{sign, term};

/// Courtesy of Wikipedia:
/// Gives the greatest common denominator of the two inputs, unless that's 2³¹.
/// 2³¹ doesn't fit in an `i32`, so it returns -2³¹, which does.
fn gcd(u: i32, v: i32) -> i32 {
    // `wrapping_abs` gives a number's absolute value, unless that's 2³¹. 2³¹
    // won't fit in `i32`, so it gives -2³¹ instead.
    let mut v = v.wrapping_abs() as u32;
    if u == 0 {
        return v as i32;
    }
    let mut u = u.wrapping_abs() as u32;
    if v == 0 {
        return u as i32;
    }

    // `|` is bitwise OR. `trailing_zeros` quickly counts a binary number's
    // trailing zeros, giving its prime factorization's exponent on two.
    let gcd_exponent_on_two = (u | v).trailing_zeros();

    // `>>=` divides the left by two to the power of the right, storing that in
    // the left variable. `u` divided by its prime factorization's power of two
    // turns it odd.
    u >>= u.trailing_zeros();
    v >>= v.trailing_zeros();

    while u != v {
        if u < v {
            // Swap the variables' values with each other.
            core::mem::swap(&mut u, &mut v);
        }
        u -= v;
        u >>= u.trailing_zeros();
    }

    // `<<` multiplies the left by two to the power of the right.
    (u << gcd_exponent_on_two) as i32
}

#[derive(Copy, Clone)]
pub(crate) struct Fraction {
    numerator: i32,
    denominator: i32
}

pub(crate) enum ExtendedReal {
    RealNumber(Fraction),
    Infinity(sign::Sign)
}

impl ExtendedReal {
    pub(crate) fn to_string(&self) -> String {
        match self {
            ExtendedReal::RealNumber(n) => n.to_string(),
            ExtendedReal::Infinity(inf) => inf.to_string().to_owned() + "∞",
        }
    }
}

impl Fraction {
    pub(crate) fn to_string(&self) -> String {
        if self.numerator == 0 { // 0/1
            0.to_string()
        } else if self.numerator == self.denominator { // 1/1
            self.numerator.to_string()
        } else {
            self.numerator.to_string() + "/" + &self.denominator.to_string()
        }
    }

    pub(crate) fn to_float(&self) -> f32 {
        (self.numerator as f32) / (self.denominator as f32)
    }
}

pub(crate) fn fraction_from_float(n: f32) -> Fraction {
    let mut a = n;
    let mut b: f32 = 1.0;

    /* Keep multiplying a and b by by 10
     * until a (the numerator) is a whole
     * number.
     */
    while (a % 10.0) != 0.0 {
        a *= 10.0;
        b *= 10.0;
    }

    /* Find and divide by gcd */
    let greatest_common_div: i32    = gcd(a as i32, b as i32);
    let numerator: i32              = a as i32 / greatest_common_div;
    let denominator: i32            = b as i32 / greatest_common_div;
    
    Fraction { numerator, denominator }
}