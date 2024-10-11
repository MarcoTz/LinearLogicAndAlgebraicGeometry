use std::ops::{Add, Mul, Neg};

/// Abelian Group
/// Elements need to be able to be added
/// implicitly this means the group product is commutative
/// Elements need to be able to be negated This is the group inverse
pub trait AbelianGroup
where
    Self: Add<Output = Self>,
    Self: Neg<Output = Self>,
    Self: PartialEq,
    Self: Sized,
{
    ///neutral element of addition
    fn zero() -> Self;
}

/// Multiplicative Group
/// elements only need to be able to be multiplied
/// this does not necessarily mean the product is commutative
pub trait Group: Mul<Output = Self> + PartialEq + Sized {
    /// neutral element of mutliplication
    fn one() -> Self;
    /// inverse of an element
    /// always needs to exists
    fn inverse(self) -> Self;
}

///Subgroup of a group G
///implemented with an element check
pub struct SubGroup<G>
where
    G: Group,
{
    pub elem: Box<dyn Fn(&G) -> bool>,
}

///Abelian Subgroup of a group G
pub struct AbelianSubGroup<G>
where
    G: AbelianGroup,
{
    pub elem: Box<dyn Fn(&G) -> bool>,
}

/// Treat a group as multiplicative group
/// i.e. forget its abelian
/// simply wraps an element
#[derive(PartialEq)]
pub struct AsMultGroup<G>
where
    G: AbelianGroup,
{
    elem: G,
}

///Abelian groups can be treated as mutliplicative groups
impl<G> From<G> for AsMultGroup<G>
where
    G: AbelianGroup,
{
    fn from(g: G) -> AsMultGroup<G> {
        AsMultGroup { elem: g }
    }
}

impl<G> Mul for AsMultGroup<G>
where
    G: AbelianGroup,
{
    type Output = Self;
    fn mul(self, other: AsMultGroup<G>) -> Self {
        (self.elem + other.elem).into()
    }
}

impl<G> Group for AsMultGroup<G>
where
    G: AbelianGroup,
{
    fn one() -> Self {
        G::zero().into()
    }

    fn inverse(self) -> Self {
        (-self.elem).into()
    }
}
