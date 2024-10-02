use std::fmt;

pub trait Set: Sized {
    type Element: fmt::Debug + Clone;
}

impl<T: Set, U: Set> Set for (T, U) {
    type Element = (T::Element, U::Element);
}

pub trait Subset<Super>
where
    Self: Set,
    Super: Set,
{
    fn embed(x: Self::Element) -> <Super as Set>::Element;
}

impl<T: Set> Subset<T> for T {
    fn embed(x: T::Element) -> T::Element {
        x
    }
}
