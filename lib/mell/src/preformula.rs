use super::formula::Formula;
use std::{fmt, rc::Rc};

pub type Atom = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Pos,
    Neg,
}

impl Polarity {
    pub fn flip(self) -> Polarity {
        match self {
            Polarity::Pos => Polarity::Neg,
            Polarity::Neg => Polarity::Pos,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientedAtom {
    pub atom: Atom,
    pub pol: Polarity,
}

impl OrientedAtom {
    pub fn flip(self) -> OrientedAtom {
        OrientedAtom {
            atom: self.atom,
            pol: self.pol.flip(),
        }
    }
}

#[derive(Clone)]
pub enum Preformula {
    Atomic(OrientedAtom),
    Tensor(Rc<Preformula>, Rc<Preformula>),
    Par(Rc<Preformula>, Rc<Preformula>),
    Neg(Rc<Preformula>),
    Bang(Rc<Preformula>),
    Quest(Rc<Preformula>),
}

impl From<Formula> for Preformula {
    fn from(f: Formula) -> Preformula {
        match f {
            Formula::Atomic(at) => Preformula::Atomic(at),
            Formula::Tensor(l, r) => Preformula::Tensor(
                Rc::new(Rc::unwrap_or_clone(l).into()),
                Rc::new(Rc::unwrap_or_clone(r).into()),
            ),
            Formula::Par(l, r) => Preformula::Par(
                Rc::new(Rc::unwrap_or_clone(l).into()),
                Rc::new(Rc::unwrap_or_clone(r).into()),
            ),
            Formula::Bang(p) => Preformula::Bang(Rc::new(Rc::unwrap_or_clone(p).into())),
            Formula::Quest(p) => Preformula::Quest(Rc::new(Rc::unwrap_or_clone(p).into())),
        }
    }
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Polarity::Pos => f.write_str("+"),
            Polarity::Neg => f.write_str("-"),
        }
    }
}

impl fmt::Display for OrientedAtom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.pol, self.atom)
    }
}
impl fmt::Display for Preformula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Preformula::Atomic(at) => at.fmt(f),
            Preformula::Tensor(l, r) => write!(f, "{}⊗{}", l, r),
            Preformula::Par(l, r) => write!(f, "{}⅋ {}", l, r),
            Preformula::Neg(p) => write!(f, "¬{}", p),
            Preformula::Bang(p) => write!(f, "!{}", p),
            Preformula::Quest(p) => write!(f, "?{}", p),
        }
    }
}
