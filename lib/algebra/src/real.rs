use super::{field::Field, function::BinOp, group::Group, ring::Ring, set::Set};

#[derive(Debug)]
pub struct Real;
pub struct RealProd;
pub struct RealSum;

impl Set for Real {
    type Element = f32;
}

impl BinOp<Real> for RealProd {
    fn apply(&self, a: f32, b: f32) -> f32 {
        a * b
    }
}
impl BinOp<Real> for RealSum {
    fn apply(&self, a: f32, b: f32) -> f32 {
        a + b
    }
}
impl Group<RealSum> for Real {
    fn inverse(a: f32) -> f32 {
        -a
    }
}
impl Group<RealProd> for Real {
    fn inverse(a: f32) -> f32 {
        1.0 / a
    }
}

impl Ring<RealProd, RealSum> for Real {}
impl Field<RealProd, RealSum> for Real {}
