use super::{errors::Error, group::Group, ring::Ring};
use std::ops::{Add, Mul};

pub trait Field: Ring + Clone {
    fn one() -> Self;
    fn inverse(self) -> Self;
    fn divide(self, other: Self) -> Result<Self, Error> {
        if self == Self::zero() {
            Err(Error::DivisionByZero)
        } else {
            Ok(self * other.inverse())
        }
    }
}
