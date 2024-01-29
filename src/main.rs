#![allow(unused)]

use polynomial::Trinominal;

use crate::fraction::fraction_from_float;


pub mod fraction;
pub mod term;
pub mod polynomial;
pub mod sign;
pub mod domain;

fn main() {
    // Simple test program

    // f(x) = x^2
    let f: polynomial::Trinominal = polynomial::Trinominal {
        a: fraction_from_float(1.0),
        b: fraction_from_float(0.0),
        c: fraction_from_float(0.0),
    };

    let d = domain::dom(polynomial::Polynomial::Trinominal(f)).to_string();
    println!("Domain: {}", d);

    let s = domain::sym(polynomial::Polynomial::Trinominal(f));
    println!("Reflection symmetry: {}", s.0);
    println!("Rotational symmetry: {}", s.1);

    let z = f.zero_values();
    println!("Zero values: {{{}, {}}}", z.0.to_string(), z.1.to_string());
}
