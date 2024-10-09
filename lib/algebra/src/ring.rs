use super::group::AbelianGroup;
use std::ops::Mul;

pub trait Ring
where
    Self: AbelianGroup,
    Self: Mul<Output = Self>,
{
    fn one() -> Self;
    fn pow(self, n: u32) -> Self
    where
        Self: Clone,
    {
        if n == 0 {
            Self::one()
        } else {
            self.clone() * (self.pow(n - 1))
        }
    }
}
