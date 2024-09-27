use std::fmt;

pub trait Set: Sized + fmt::Debug {
    type Element: fmt::Debug;
    fn element(&self, e: &Self::Element) -> bool;
}

impl<T: Set> Set for (T, T) {
    type Element = (T::Element, T::Element);
    fn element(&self, e: &Self::Element) -> bool {
        self.0.element(&e.0) && self.1.element(&e.1)
    }
}

pub trait Function {
    type Domain: Set;
    type Codomain: Set;

    fn apply(x: <Self::Domain as Set>::Element) -> <Self::Codomain as Set>::Element;
}

pub trait BinOp<S: Set>: Function<Domain = (S, S), Codomain = S> {}
