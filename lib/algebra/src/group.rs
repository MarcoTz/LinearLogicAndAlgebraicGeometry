use std::ops::{Add, Mul};

pub trait AbelianGroup: Add<Output = Self> + PartialEq + Sized {
    fn zero() -> Self;
    fn neg(self) -> Self;
}

pub trait Group: Mul<Output = Self> + PartialEq + Sized {
    fn one() -> Self;
    fn inverse(self) -> Self;
}
