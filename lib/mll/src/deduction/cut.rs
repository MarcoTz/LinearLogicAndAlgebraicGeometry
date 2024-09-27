use super::{Deduction, DeductionRule};
use crate::{formula::Formula, sequent::Sequent};
use std::ops::Neg;

#[derive(Clone, PartialEq, Eq)]
pub struct Cut {
    left_left: Sequent,
    left_right: Sequent,
    right_left: Sequent,
    right_right: Sequent,
    active: Formula,
}

impl Deduction for Cut {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut left = self.left_left.to_owned();
        left.push(self.active.to_owned());
        left.extend(self.left_right.to_owned());
        let mut right = self.right_left.to_owned();
        right.push(self.active.to_owned().neg());
        right.extend(self.right_right.to_owned());
        vec![left, right]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.left_left.to_owned();
        conc.extend(self.left_right.to_owned());
        conc.extend(self.right_left.to_owned());
        conc.extend(self.right_right.to_owned());
        conc
    }

    fn get_active(&self) -> Vec<Formula> {
        vec![self.active.to_owned(), self.active.to_owned().neg()]
    }
}

impl From<Cut> for DeductionRule {
    fn from(cut: Cut) -> DeductionRule {
        DeductionRule::Cut(cut)
    }
}
