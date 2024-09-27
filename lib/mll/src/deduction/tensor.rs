use super::{Deduction, DeductionRule};
use crate::{formula::Formula, sequent::Sequent};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct Tensor {
    left_left: Sequent,
    active_left: Formula,
    left_right: Sequent,
    right_left: Sequent,
    active_right: Formula,
    right_right: Sequent,
}

impl Deduction for Tensor {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut left = self.left_left.to_owned();
        left.push(self.active_left.to_owned());
        left.extend(self.left_right.to_owned());

        let mut right = self.right_left.to_owned();
        right.push(self.active_right.to_owned());
        right.extend(self.right_right.to_owned());

        vec![left, right]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.left_left.to_owned();
        conc.extend(self.left_right.to_owned());
        conc.push(Formula::Tensor(
            Rc::new(self.active_left.to_owned()),
            Rc::new(self.active_right.to_owned()),
        ));
        conc.extend(self.right_left.to_owned());
        conc.extend(self.right_right.to_owned());
        conc
    }

    fn get_active(&self) -> Vec<Formula> {
        vec![self.active_left.to_owned(), self.active_right.to_owned()]
    }
}

impl From<Tensor> for DeductionRule {
    fn from(tensor: Tensor) -> DeductionRule {
        DeductionRule::Tensor(tensor)
    }
}
