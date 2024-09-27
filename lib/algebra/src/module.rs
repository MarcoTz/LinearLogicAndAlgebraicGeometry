use super::{group::Group, ring::Ring, set::BinOp};

pub trait LeftModule<
    RingSum: BinOp<R>,
    RingProd: BinOp<R>,
    U,
    Sum: BinOp<Self>,
    R: Ring<RingSum, RingProd>,
>: Group<Sum>
{
    fn module_prod(r: R::Element, x: Self::Element) -> Self::Element;
}
