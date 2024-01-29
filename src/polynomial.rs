use std::ops::Index;
use crate::domain::Domain;
use crate::domain::RangeOperator;
use crate::fraction::fraction_from_float;
use crate::fraction::ExtendedReal;
use crate::fraction::Fraction;
use crate::sign::Sign;
use crate::term;
use crate::fraction;
use crate::sign;


pub(crate) trait Polynomial {
    /// # Arguments
    /// None
    /// 
    /// # Returns
    /// The domain of the function
    /// 
    fn domain(&self) -> Domain;

    /// # Arguments
    /// None
    /// 
    /// # Returns
    /// A tuple containing the two zero values
    /// of the Trinominal
    /// 
    fn zero_values(&self) -> (Fraction, Fraction);

    /// # Arguments
    /// x : The value to compute
    /// 
    /// # Returns
    /// f(x)
    /// 
    fn compute(&self, x: Fraction) -> f32;

    /// # Arguments
    /// None
    /// 
    /// ### Returns
    /// (bool, bool) for Reflection Symmetry and  Rotational
    /// Symmetry of 180 degrees respectively
    /// 
    /// ### Note: 
    /// This does not "prove" symmetry, but merely
    /// estimates wether or not a given function
    /// is symmetric or not.
    fn symmetry(&self) -> (bool, bool);

    fn to_string(&self) -> String;
}
