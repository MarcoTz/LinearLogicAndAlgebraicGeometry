use super::{function::BinOp, set::Set};

pub trait Monoid<Op>
where
    Self: Set,
    Op: BinOp<Self>,
{
    fn prod(a: Self::Element, b: Self::Element) -> Self::Element;
    fn identity() -> Self::Element;
}
