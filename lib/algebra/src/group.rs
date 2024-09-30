use super::{function::BinOp, monoid::Monoid};

pub trait Group<Op>
where
    Self: Monoid<Op>,
    Op: BinOp<Self>,
{
    fn identity() -> Self::Element {
        <Self as Monoid<Op>>::identity()
    }
    fn prod(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Monoid<Op>>::prod(a, b)
    }
    fn inverse(a: Self::Element) -> Self::Element;
}

impl<T, Op> Monoid<Op> for T
where
    T: Group<Op>,
    Op: BinOp<T>,
{
    fn prod(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Group<Op>>::prod(a, b)
    }

    fn identity() -> Self::Element {
        <Self as Group<Op>>::identity()
    }
}
