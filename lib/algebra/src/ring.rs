use super::group::Group;
use std::ops::{Add, Mul};

pub trait Ring: Group + Mul<Output = Self> + Clone {
    fn one() -> Self;
    fn pow(self, n: u32) -> Self {
        if n == 0 {
            Self::one()
        } else {
            self.clone() * (self.pow(n - 1))
        }
    }
}

#[derive(PartialEq)]
pub struct AsGroup<T: Ring> {
    elem: T,
}

impl<T: Ring> From<T> for AsGroup<T> {
    fn from(elem: T) -> AsGroup<T> {
        AsGroup { elem }
    }
}

impl<T: Ring> Add for AsGroup<T> {
    type Output = AsGroup<T>;
    fn add(self, other: AsGroup<T>) -> AsGroup<T> {
        AsGroup {
            elem: self.elem + other.elem,
        }
    }
}

impl<T: Ring> Group for AsGroup<T> {
    fn zero() -> AsGroup<T> {
        AsGroup {
            elem: <T as Group>::zero(),
        }
    }
    fn neg(self) -> AsGroup<T> {
        AsGroup {
            elem: <T as Group>::neg(self.elem),
        }
    }
}
