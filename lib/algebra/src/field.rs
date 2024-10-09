use super::ring::Ring;
use std::ops::Div;

pub trait Field: Ring + Div<Output = Self> {
    fn one() -> Self;
    fn inverse(self) -> Self;
}
