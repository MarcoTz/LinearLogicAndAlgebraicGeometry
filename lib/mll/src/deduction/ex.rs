use super::{Deduction, DeductionRule};
use crate::{formula::Formula, sequent::Sequent};

#[derive(Clone, PartialEq, Eq)]
pub struct Ex {
    prem_left: Sequent,
    active_left: Formula,
    active_right: Formula,
    prem_right: Sequent,
}

impl Deduction for Ex {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut prem = self.prem_left.to_owned();
        prem.push(self.active_left.to_owned());
        prem.push(self.active_right.to_owned());
        prem.extend(self.prem_right.to_owned());
        vec![prem]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.prem_left.to_owned();
        conc.push(self.active_right.to_owned());
        conc.push(self.active_left.to_owned());
        conc.extend(self.prem_right.to_owned());
        conc
    }

    fn get_active(&self) -> Vec<Formula> {
        vec![self.active_left.to_owned(), self.active_right.to_owned()]
    }
}

impl From<Ex> for DeductionRule {
    fn from(ex: Ex) -> DeductionRule {
        DeductionRule::Ex(ex)
    }
}
