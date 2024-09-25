use std::fmt;

pub type Atom = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Polarity {
    Pos,
    Neg,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrientedAtom {
    pub atom: Atom,
    pub pol: Polarity,
}

impl Polarity {
    pub fn flip(self) -> Polarity {
        match self {
            Polarity::Pos => Polarity::Neg,
            Polarity::Neg => Polarity::Pos,
        }
    }
}

impl OrientedAtom {
    pub fn flip(self) -> OrientedAtom {
        OrientedAtom {
            atom: self.atom,
            pol: self.pol.flip(),
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
