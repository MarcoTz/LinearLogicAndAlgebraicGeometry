use std::ops::Add;

pub trait Group: Add<Output = Self> + PartialEq + Sized {
    fn zero() -> Self;
    fn neg(self) -> Self;
}
