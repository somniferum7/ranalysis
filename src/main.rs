#![allow(unused)]

pub mod fraction;
pub mod term;

fn main() {
    let term = term::Term {
        variable: 'x',
        coefficient: fraction::fraction_from_float(0.5),
        exponent: 2,
    };

    println!("{}", term.to_string());
}
