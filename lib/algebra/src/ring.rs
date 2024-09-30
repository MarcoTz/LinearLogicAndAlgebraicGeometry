use super::{function::BinOp, group::Group, monoid::Monoid};

pub trait Ring<Prod, Sum>
where
    Self: Group<Sum>,
    Self: Monoid<Prod>,
    Prod: BinOp<Self>,
    Sum: BinOp<Self>,
{
    fn add(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Group<Sum>>::prod(a, b)
    }
    fn zero() -> Self::Element {
        <Self as Group<Sum>>::identity()
    }
    fn mult(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Monoid<Prod>>::prod(a, b)
    }
    fn one() -> Self::Element {
        <Self as Monoid<Prod>>::identity()
    }
    fn neg(a: Self::Element) -> Self::Element {
        <Self as Group<Sum>>::inverse(a)
    }
}
