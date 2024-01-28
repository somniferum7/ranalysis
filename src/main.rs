#![allow(unused)]

use crate::fraction::fraction_from_float;

pub mod fraction;
pub mod term;
pub mod polynomial;
pub mod sign;

fn main() {
    let term = term::Term {
        variable: 'x',
        coefficient: fraction::fraction_from_float(0.5),
        exponent: 2,
    };

    let f = polynomial::Trinominal {
        a: fraction_from_float(1.0),
        b: fraction_from_float(5.0),
        c: fraction_from_float(6.0),
    };

    println!("{}", term.to_string());
    println!("{}, {}", f.zero_values().0.to_string(), f.zero_values().1.to_string());
}
