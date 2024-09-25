use super::errors::Error;
use common::definitions::OrientedAtom;
use std::{fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Formula {
    Atomic(OrientedAtom),
    Tensor(Rc<Formula>, Rc<Formula>),
    Impl(Rc<Formula>, Rc<Formula>),
    Neg(Rc<Formula>),
    Bang(Rc<Formula>),
}
#[derive(Debug, Clone)]
pub struct Sequent {
    pub antecent: Vec<Formula>,
    pub succedent: Formula,
}

pub struct SplitSequent {
    pub left_ante: Vec<Formula>,
    pub active: Formula,
    pub right_ante: Vec<Formula>,
    pub succedent: Formula,
}

pub fn split_seq(active: &Formula, s: Sequent) -> Result<SplitSequent, Error> {
    let mut active_found = None;
    let mut right = vec![];
    let mut left = vec![];
    let mut go_right = false;
    let err = Error::ActiveNotFound(active.to_owned(), s.clone());
    for form in s.antecent.into_iter() {
        if form == *active {
            active_found = Some(form);
            go_right = true;
        } else if go_right {
            right.push(form);
        } else {
            left.push(form);
        }
    }

    match active_found {
        Some(form) => Ok(SplitSequent {
            left_ante: left,
            active: form,
            right_ante: right,
            succedent: s.succedent,
        }),
        None => Err(err),
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Atomic(atom) => atom.fmt(f),
            Formula::Tensor(l, r) => write!(f, "{}⊗{}", l, r),
            Formula::Impl(l, r) => todo!("{}⊸{}", l, r),
            Formula::Neg(form) => write!(f, "¬{}", form),
            Formula::Bang(form) => write!(f, "!{form}"),
        }
    }
}

impl fmt::Display for Sequent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ant_strs: Vec<String> = self.antecent.iter().map(|f| format!("{}", f)).collect();
        write!(f, "{} |- {}", ant_strs.join(","), self.succedent)
    }
}
