#![allow(unused)]

use trinomial::Trinomial;

use crate::{fraction::fraction_from_float, polynomial::Polynomial};


pub mod fraction;
pub mod term;
pub mod polynomial;
pub mod sign;
pub mod domain;
pub mod trinomial;

fn main() {
    // Simple test program

    // f(x) = x^2
    let f: Trinomial = Trinomial {
        a: fraction_from_float(1.0),
        b: fraction_from_float(0.0),
        c: fraction_from_float(0.0),
    };

    let d = f.domain();
    println!("Domain: {}", d);

    let s = f.symmetry();
    println!("Reflection symmetry: {}", s.0);
    println!("Rotational symmetry: {}", s.1);

    let z = f.zero_values();
    println!("Zero values: {{{}, {}}}", z.0.to_string(), z.1.to_string());
}
