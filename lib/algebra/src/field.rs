use super::{function::BinOp, group::Group, ring::Ring};

pub trait Field<Prod, Sum>
where
    Self: Ring<Prod, Sum>,
    Self: Group<Prod>,
    Prod: BinOp<Self>,
    Sum: BinOp<Self>,
{
    fn one() -> Self::Element {
        <Self as Ring<Prod, Sum>>::one()
    }
    fn add(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Ring<Prod, Sum>>::add(a, b)
    }
    fn zero() -> Self::Element {
        <Self as Ring<Prod, Sum>>::zero()
    }
    fn mult(a: Self::Element, b: Self::Element) -> Self::Element {
        <Self as Ring<Prod, Sum>>::mult(a, b)
    }
    fn neg(a: Self::Element) -> Self::Element {
        <Self as Ring<Prod, Sum>>::neg(a)
    }
    fn inverse(a: Self::Element) -> Self::Element {
        <Self as Group<Prod>>::inverse(a)
    }
}
