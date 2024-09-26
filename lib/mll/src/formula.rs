use super::preformula::Preformula;
use common::definitions::OrientedAtom;
use std::{fmt, ops::Neg, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Atomic(OrientedAtom),
    Tensor(Rc<Formula>, Rc<Formula>),
    Par(Rc<Formula>, Rc<Formula>),
    Bang(Rc<Formula>),
    Quest(Rc<Formula>),
}

impl Neg for Formula {
    type Output = Formula;
    fn neg(self) -> Self::Output {
        let pr: Preformula = self.into();
        Preformula::Neg(Rc::new(pr)).into()
    }
}

impl Formula {
    pub fn depth(&self) -> i32 {
        match self {
            Formula::Atomic(_) => 0,
            Formula::Tensor(l, r) => l.depth().max(r.depth()),
            Formula::Par(l, r) => l.depth().max(r.depth()),
            Formula::Bang(form) => form.depth() + 1,
            Formula::Quest(form) => form.depth() + 1,
        }
    }

    pub fn is_linear(&self) -> bool {
        self.depth() == 0
    }

    pub fn is_shallow(&self) -> bool {
        self.depth() <= 1
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Atomic(at) => at.fmt(f),
            Formula::Tensor(l, r) => write!(f, "{}⊗{}", l, r),
            Formula::Par(l, r) => write!(f, "{}⅋ {}", l, r),
            Formula::Bang(form) => write!(f, "!{}", form),
            Formula::Quest(form) => write!(f, "?{}", form),
        }
    }
}

impl From<Preformula> for Formula {
    fn from(pr: Preformula) -> Formula {
        match pr {
            Preformula::Atomic(at) => Formula::Atomic(at),
            Preformula::Tensor(l, r) => Formula::Tensor(
                Rc::new(Rc::unwrap_or_clone(l).into()),
                Rc::new(Rc::unwrap_or_clone(r).into()),
            ),
            Preformula::Par(l, r) => Formula::Par(
                Rc::new(Rc::unwrap_or_clone(l).into()),
                Rc::new(Rc::unwrap_or_clone(r).into()),
            ),
            Preformula::Bang(p) => Formula::Bang(Rc::new(Rc::unwrap_or_clone(p).into())),
            Preformula::Quest(p) => Formula::Quest(Rc::new(Rc::unwrap_or_clone(p).into())),
            Preformula::Neg(p) => match Rc::unwrap_or_clone(p) {
                Preformula::Atomic(at) => Formula::Atomic(at.flip()),
                Preformula::Tensor(l, r) => {
                    let l_neg: Formula = Preformula::Neg(l).into();
                    let r_neg: Formula = Preformula::Neg(r).into();
                    Formula::Par(Rc::new(l_neg), Rc::new(r_neg))
                }
                Preformula::Par(l, r) => {
                    let l_neg: Formula = Preformula::Neg(l).into();
                    let r_neg: Formula = Preformula::Neg(r).into();
                    Formula::Tensor(Rc::new(l_neg), Rc::new(r_neg))
                }
                Preformula::Bang(p) => {
                    let p_neg: Formula = Preformula::Neg(p).into();
                    Formula::Quest(Rc::new(p_neg))
                }
                Preformula::Quest(p) => {
                    let p_neg: Formula = Preformula::Neg(p).into();
                    Formula::Bang(Rc::new(p_neg))
                }
                Preformula::Neg(p) => {
                    let p_neg: Formula = Preformula::Neg(p).into();
                    let p_neg_pre: Preformula = p_neg.into();
                    Preformula::Neg(Rc::new(p_neg_pre)).into()
                }
            },
        }
    }
}

impl From<OrientedAtom> for Formula {
    fn from(at: OrientedAtom) -> Formula {
        Formula::Atomic(at)
    }
}
