use super::group::Group;

pub trait Ring: PartialEq {
    fn zero() -> Self;
    fn neg(self) -> Self;
    fn add(self, other: Self) -> Self;
    fn one() -> Self;
    fn mult(self, other: Self) -> Self;
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

impl<T: Ring> Group for AsGroup<T> {
    fn zero() -> AsGroup<T> {
        AsGroup {
            elem: <T as Ring>::zero(),
        }
    }
    fn neg(self) -> AsGroup<T> {
        AsGroup {
            elem: <T as Ring>::neg(self.elem),
        }
    }
    fn add(self, other: AsGroup<T>) -> AsGroup<T> {
        AsGroup {
            elem: <T as Ring>::add(self.elem, other.elem),
        }
    }
}
