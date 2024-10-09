use std::ops::{Add, Mul, Neg};

pub trait AbelianGroup
where
    Self: Add<Output = Self>,
    Self: Neg<Output = Self>,
    Self: PartialEq,
    Self: Sized,
{
    fn zero() -> Self;
}

pub trait Group: Mul<Output = Self> + PartialEq + Sized {
    fn one() -> Self;
    fn inverse(self) -> Self;
}
