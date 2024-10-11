use super::group::{AbelianGroup, AbelianSubGroup};
use std::ops::Mul;

///Rings
///Rings are abelian groups with multiplication and identity
///(all rings are assumed to have neutral element)
///the abelian group and multiplication requirements are ensured by the trait bounds
///the neutral elemenent ist the only method that has to be implemented
pub trait Ring
where
    Self: AbelianGroup,
    Self: Mul<Output = Self>,
{
    /// The neutral element of multiplication
    fn one() -> Self;
    /// x^n
    /// requires cloning elements
    fn pow(self, n: u32) -> Self
    where
        Self: Clone,
    {
        if n == 0 {
            Self::one()
        } else {
            self.clone() * (self.pow(n - 1))
        }
    }
}

///Subring of a ring R
///implmented as a element check
pub struct SubRing<R>
where
    R: Ring,
{
    pub elem: Box<dyn Fn(&R) -> bool>,
}

///Every subring is also an abelian subgroup
impl<R: Ring> SubRing<R> {
    pub fn as_subgroup(self) -> AbelianSubGroup<R> {
        AbelianSubGroup { elem: self.elem }
    }
}
