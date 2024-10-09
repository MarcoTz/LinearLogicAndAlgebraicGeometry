use super::group::Group;
use std::ops::{Add, Mul};

pub trait Ring: Group + Mul<Output = Self> + Clone {
    fn one() -> Self;
    fn pow(self, n: u32) -> Self {
        if n == 0 {
            Self::one()
        } else {
            self.clone() * (self.pow(n - 1))
        }
    }
}
