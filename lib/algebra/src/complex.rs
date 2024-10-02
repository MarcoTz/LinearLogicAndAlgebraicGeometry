use super::{field::Field, function::BinOp, group::Group, ring::Ring, set::Set};

#[derive(Debug, PartialEq, Clone)]
pub struct ComplexNum {
    re: f32,
    im: f32,
}

impl ComplexNum {
    fn angle(&self) -> f32 {
        (self.im / self.re).atan()
    }
    fn arg(&self) -> f32 {
        (self.im * self.im + self.re * self.re).sqrt()
    }
    fn from_polar(angle: f32, arg: f32) -> Self {
        ComplexNum {
            re: angle.sin() * arg,
            im: angle.cos() * arg,
        }
    }
}

#[derive(Debug)]
pub struct Complex;
pub struct ComplexProd;
pub struct ComplexSum;

impl Set for Complex {
    type Element = ComplexNum;
}
impl BinOp<Complex> for ComplexProd {
    fn apply(&self, a: ComplexNum, b: ComplexNum) -> ComplexNum {
        ComplexNum {
            re: a.re * b.re - a.im * b.im,
            im: a.re * b.im + a.im * b.re,
        }
    }
}
impl BinOp<Complex> for ComplexSum {
    fn apply(&self, a: ComplexNum, b: ComplexNum) -> ComplexNum {
        ComplexNum {
            re: a.re + b.re,
            im: a.im + b.im,
        }
    }
}

impl Group<ComplexSum> for Complex {
    fn inverse(a: ComplexNum) -> ComplexNum {
        ComplexNum {
            re: -a.re,
            im: -a.im,
        }
    }
}

impl Group<ComplexProd> for Complex {
    fn inverse(a: ComplexNum) -> ComplexNum {
        ComplexNum::from_polar(-a.angle(), 1.0 / a.arg())
    }
}

impl Ring<ComplexProd, ComplexSum> for Complex {}
impl Field<ComplexProd, ComplexSum> for Complex {}
