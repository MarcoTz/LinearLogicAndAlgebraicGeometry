use super::{errors::Error, group::Group, ring::Ring};
use std::ops::{Add, Mul};

pub trait Field: Ring + Clone {
    fn one() -> Self;
    fn inverse(self) -> Self;
    fn divide(self, other: Self) -> Result<Self, Error> {
        if self == Self::zero() {
            Err(Error::DivisionByZero)
        } else {
            Ok(self * other.inverse())
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct AsRing<T: Field> {
    elem: T,
}

impl<T: Field> Add for AsRing<T> {
    type Output = AsRing<T>;
    fn add(self, other: AsRing<T>) -> Self {
        AsRing {
            elem: self.elem + other.elem,
        }
    }
}
impl<T: Field> Mul for AsRing<T> {
    type Output = AsRing<T>;
    fn mul(self, other: Self) -> Self {
        AsRing {
            elem: self.elem * other.elem,
        }
    }
}

impl<T: Field> Group for AsRing<T> {
    fn zero() -> Self {
        AsRing {
            elem: <T as Group>::zero(),
        }
    }
    fn neg(self) -> Self {
        AsRing {
            elem: self.elem.neg(),
        }
    }
}
impl<T: Field> Ring for AsRing<T> {
    fn one() -> Self {
        AsRing {
            elem: <T as Field>::one(),
        }
    }
}

impl<T: Field> From<T> for AsRing<T> {
    fn from(elem: T) -> AsRing<T> {
        AsRing { elem }
    }
}

#[derive(PartialEq)]
pub struct AsAdditiveGroup<T: Field> {
    elem: T,
}

impl<T: Field> Add for AsAdditiveGroup<T> {
    type Output = AsAdditiveGroup<T>;
    fn add(self, other: Self) -> Self {
        AsAdditiveGroup {
            elem: self.elem + other.elem,
        }
    }
}

impl<T: Field> Group for AsAdditiveGroup<T> {
    fn zero() -> Self {
        AsAdditiveGroup {
            elem: <T as Group>::zero(),
        }
    }
    fn neg(self) -> Self {
        AsAdditiveGroup {
            elem: <T as Group>::neg(self.elem),
        }
    }
}
