use super::{Deduction, DeductionRule};
use crate::{formula::Formula, sequent::Sequent};
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
pub struct Par {
    prem_left: Sequent,
    active_left: Formula,
    active_right: Formula,
    prem_right: Sequent,
}

impl Deduction for Par {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut prem = self.prem_left.to_owned();
        prem.push(self.active_left.to_owned());
        prem.push(self.active_right.to_owned());
        prem.extend(self.prem_right.to_owned());
        vec![prem]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.prem_left.to_owned();
        conc.push(Formula::Par(
            Rc::new(self.active_left.to_owned()),
            Rc::new(self.active_right.to_owned()),
        ));
        conc.extend(self.prem_right.to_owned());
        conc
    }

    fn get_active(&self) -> Sequent {
        vec![self.active_left.to_owned(), self.active_right.to_owned()]
    }
}

impl From<Par> for DeductionRule {
    fn from(par: Par) -> DeductionRule {
        DeductionRule::Par(par)
    }
}
