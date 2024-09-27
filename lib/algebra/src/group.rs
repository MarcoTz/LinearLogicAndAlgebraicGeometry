use super::{monoid::Monoid, set::BinOp};
use std::error::Error;

pub trait Group<Op: BinOp<Self>>: Monoid<Op> {
    fn identity() -> Self::Element {
        <Self as Monoid<Op>>::identity()
    }
    fn prod(&self, a: Self::Element, b: Self::Element) -> Result<Self::Element, Box<dyn Error>> {
        <Self as Monoid<Op>>::prod(self, a, b)
    }
    fn inverse(&self, a: Self::Element) -> Result<Self::Element, Box<dyn Error>>;
}
