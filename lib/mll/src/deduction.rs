use super::{formula::Formula, sequent::Sequent};
use std::{ops::Neg, rc::Rc};

pub trait Deduction {
    fn get_premises(&self) -> Vec<Sequent>;
    fn get_conclusion(&self) -> Sequent;
}

pub enum DeductionRule {
    Ax(Ax),
    Cut(Cut),
    Tensor(Tensor),
    Par(Par),
    Ex(Ex),
}

impl Deduction for DeductionRule {
    fn get_premises(&self) -> Vec<Sequent> {
        match self {
            DeductionRule::Ax(ax) => ax.get_premises(),
            DeductionRule::Cut(cut) => cut.get_premises(),
            DeductionRule::Tensor(tensor) => tensor.get_premises(),
            DeductionRule::Par(par) => par.get_premises(),
            DeductionRule::Ex(ex) => ex.get_premises(),
        }
    }

    fn get_conclusion(&self) -> Sequent {
        match self {
            DeductionRule::Ax(ax) => ax.get_conclusion(),
            DeductionRule::Cut(cut) => cut.get_conclusion(),
            DeductionRule::Tensor(tensor) => tensor.get_conclusion(),
            DeductionRule::Par(par) => par.get_conclusion(),
            DeductionRule::Ex(ex) => ex.get_conclusion(),
        }
    }
}

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
}

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
}

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
}

pub struct Par {
    prem_left: Sequent,
    active_a: Formula,
    active_b: Formula,
    prem_right: Sequent,
}

impl Deduction for Par {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut prem = self.prem_left.to_owned();
        prem.push(self.active_a.to_owned());
        prem.push(self.active_b.to_owned());
        prem.extend(self.prem_right.to_owned());
        vec![prem]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.prem_left.to_owned();
        conc.push(Formula::Par(
            Rc::new(self.active_a.to_owned()),
            Rc::new(self.active_b.to_owned()),
        ));
        conc.extend(self.prem_right.to_owned());
        conc
    }
}

pub struct Ex {
    prem_left: Sequent,
    active_a: Formula,
    active_b: Formula,
    prem_right: Sequent,
}

impl Deduction for Ex {
    fn get_premises(&self) -> Vec<Sequent> {
        let mut prem = self.prem_left.to_owned();
        prem.push(self.active_a.to_owned());
        prem.push(self.active_b.to_owned());
        prem.extend(self.prem_right.to_owned());
        vec![prem]
    }

    fn get_conclusion(&self) -> Sequent {
        let mut conc = self.prem_left.to_owned();
        conc.push(self.active_b.to_owned());
        conc.push(self.active_a.to_owned());
        conc.extend(self.prem_right.to_owned());
        conc
    }
}

impl From<Ax> for DeductionRule {
    fn from(ax: Ax) -> DeductionRule {
        DeductionRule::Ax(ax)
    }
}

impl From<Cut> for DeductionRule {
    fn from(cut: Cut) -> DeductionRule {
        DeductionRule::Cut(cut)
    }
}

impl From<Tensor> for DeductionRule {
    fn from(tensor: Tensor) -> DeductionRule {
        DeductionRule::Tensor(tensor)
    }
}

impl From<Par> for DeductionRule {
    fn from(par: Par) -> DeductionRule {
        DeductionRule::Par(par)
    }
}

impl From<Ex> for DeductionRule {
    fn from(ex: Ex) -> DeductionRule {
        DeductionRule::Ex(ex)
    }
}
