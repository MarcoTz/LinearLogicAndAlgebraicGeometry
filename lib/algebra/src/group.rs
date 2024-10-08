pub trait Group: PartialEq {
    fn zero() -> Self;
    fn neg(self) -> Self;
    fn add(self, other: Self) -> Self;
}
