use super::{
    errors::Error,
    set::{BinOp, Set},
};

pub trait Monoid<Op: BinOp<Self>>: Set {
    fn identity() -> <Self as Set>::Element;
    fn prod(
        &self,
        a: <Self as Set>::Element,
        b: <Self as Set>::Element,
    ) -> Result<<Self as Set>::Element, Box<dyn std::error::Error>> {
        if self.element(&a) {
            Ok(())
        } else {
            Err(Error::NotAnElement {
                elem: format!("{a:?}"),
                set: format!("{self:?}"),
            })
        }?;

        if self.element(&b) {
            Ok(())
        } else {
            Err(Error::NotAnElement {
                elem: format!("{b:?}"),
                set: format!("{self:?}"),
            })
        }?;
        Ok(Op::apply((a, b)))
    }
}
