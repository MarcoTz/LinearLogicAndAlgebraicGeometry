use std::ops::Add;

pub trait AbelianGroup: Add<Output = Self> + PartialEq + Sized {
    fn zero() -> Self;
    fn neg(self) -> Self;
}
