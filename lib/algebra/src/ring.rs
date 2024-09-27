use super::{group::Group, monoid::Monoid, set::BinOp};
use std::error::Error;

pub trait Ring<Sum: BinOp<Self>, Prod: BinOp<Self>>: Group<Sum> + Monoid<Prod> {
    fn zero() -> Self::Element {
        <Self as Group<Sum>>::identity()
    }

    fn one() -> Self::Element {
        <Self as Monoid<Prod>>::identity()
    }

    fn sum(&self, a: Self::Element, b: Self::Element) -> Result<Self::Element, Box<dyn Error>> {
        <Self as Group<Sum>>::prod(self, a, b)
    }

    fn prod(&self, a: Self::Element, b: Self::Element) -> Result<Self::Element, Box<dyn Error>> {
        <Self as Monoid<Prod>>::prod(self, a, b)
    }
}
