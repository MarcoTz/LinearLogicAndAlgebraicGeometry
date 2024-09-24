use super::{errors::Error, formula::Formula};

pub type Sequent = Vec<Formula>;

pub struct SplitSequent {
    pub left: Sequent,
    pub active: Formula,
    pub right: Sequent,
}
pub fn split_seq(s: Sequent, f: &Formula) -> Result<SplitSequent, Error> {
    match s.iter().position(|form| *form == *f) {
        None => Err(Error::ActiveNotFound(f.to_owned(), s)),
        Some(i) => {
            let mut new_s = s;
            let active = new_s.remove(i);
            let (left, right) = new_s.split_at(i);
            Ok(SplitSequent {
                left: left.to_vec(),
                active,
                right: right.to_vec(),
            })
        }
    }
}
