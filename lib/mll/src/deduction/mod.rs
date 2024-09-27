use super::{formula::Formula, sequent::Sequent};

mod ax;
mod cut;
mod ex;
mod par;
mod tensor;

pub use ax::Ax;
pub use cut::Cut;
pub use ex::Ex;
pub use par::Par;
pub use tensor::Tensor;

pub trait Deduction {
    fn get_premises(&self) -> Vec<Sequent>;
    fn get_conclusion(&self) -> Sequent;
    fn get_active(&self) -> Vec<Formula>;
}

#[derive(PartialEq, Eq, Clone)]
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

    fn get_active(&self) -> Vec<Formula> {
        match self {
            DeductionRule::Ax(ax) => ax.get_active(),
            DeductionRule::Cut(cut) => cut.get_active(),
            DeductionRule::Tensor(tensor) => tensor.get_active(),
            DeductionRule::Par(par) => par.get_active(),
            DeductionRule::Ex(ex) => ex.get_active(),
        }
    }
}
