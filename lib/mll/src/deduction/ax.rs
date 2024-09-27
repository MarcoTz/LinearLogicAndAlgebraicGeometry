use super::{Deduction, DeductionRule};
use crate::{formula::Formula, sequent::Sequent};
use std::ops::Neg;

#[derive(Clone, PartialEq, Eq)]
pub struct Ax {
    active: Formula,
}
impl Deduction for Ax {
    fn get_premises(&self) -> Vec<Sequent> {
        vec![]
    }
    fn get_conclusion(&self) -> Sequent {
        vec![self.active.to_owned().neg(), self.active.to_owned()]
    }
    fn get_active(&self) -> Vec<Formula> {
        vec![self.active.to_owned().neg(), self.active.to_owned()]
    }
}

impl From<Ax> for DeductionRule {
    fn from(ax: Ax) -> DeductionRule {
        DeductionRule::Ax(ax)
    }
}
