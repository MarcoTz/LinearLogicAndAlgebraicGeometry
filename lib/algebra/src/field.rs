use super::{errors::Error, group::Group, ring::Ring};

pub trait Field: Sized + PartialEq + Clone {
    fn zero() -> Self;
    fn neg(self) -> Self;
    fn add(self, other: Self) -> Self;

    fn one() -> Self;
    fn mult(self, other: Self) -> Self;
    fn inverse(self) -> Self;
    fn divide(self, other: Self) -> Result<Self, Error> {
        if self == Self::zero() {
            Err(Error::DivisionByZero)
        } else {
            Ok(self.mult(other.inverse()))
        }
    }
}

#[derive(PartialEq)]
pub struct AsRing<T: Field> {
    elem: T,
}

impl<T: Field> Ring for AsRing<T> {
    fn zero() -> Self {
        AsRing {
            elem: <T as Field>::zero(),
        }
    }
    fn neg(self) -> Self {
        AsRing {
            elem: <T as Field>::neg(self.elem),
        }
    }
    fn add(self, other: AsRing<T>) -> Self {
        AsRing {
            elem: <T as Field>::add(self.elem, other.elem),
        }
    }

    fn one() -> Self {
        AsRing {
            elem: <T as Field>::one(),
        }
    }
    fn mult(self, other: Self) -> Self {
        AsRing {
            elem: <T as Field>::mult(self.elem, other.elem),
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

impl<T: Field> Group for AsAdditiveGroup<T> {
    fn zero() -> Self {
        AsAdditiveGroup {
            elem: <T as Field>::zero(),
        }
    }
    fn neg(self) -> Self {
        AsAdditiveGroup {
            elem: <T as Field>::neg(self.elem),
        }
    }

    fn add(self, other: Self) -> Self {
        AsAdditiveGroup {
            elem: <T as Field>::add(self.elem, other.elem),
        }
    }
}
