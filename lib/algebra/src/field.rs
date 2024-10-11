use super::ring::{Ring, SubRing};
use std::ops::Div;

pub trait Field: Ring + Div<Output = Self> {
    fn one() -> Self;
    fn inverse(self) -> Self;
}

pub struct SubField<F>
where
    F: Field,
{
    pub elem: Box<dyn Fn(&F) -> bool>,
}

impl<F> From<SubField<F>> for SubRing<F>
where
    F: Field,
{
    fn from(f: SubField<F>) -> SubRing<F> {
        SubRing { elem: f.elem }
    }
}
