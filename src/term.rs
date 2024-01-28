use crate::fraction;

#[derive(Copy, Clone)]
pub struct Term {
    pub(crate) variable: char,
    pub(crate) coefficient: fraction::Fraction,
    pub(crate) exponent: u32
}

impl Term {
    pub fn to_string(&self) -> String {

        let a: String = if self.coefficient.to_float() == 1.0 {
            "".to_owned()
        } else {
            self.coefficient.to_string()
        };

        let x: String = if self.exponent == 0 {
            "".to_owned()
        } else {
            self.variable.to_string()
        };

        let exp: String = if self.exponent == 1 { "".to_owned() } else {
            self.exponent.to_string()
        };

        return a.to_owned() + &x + "^" + &exp;
    }
}