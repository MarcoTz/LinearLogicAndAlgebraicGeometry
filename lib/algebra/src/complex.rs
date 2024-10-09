use super::{field::Field, group::AbelianGroup, ring::Ring};
use std::{
    fmt,
    ops::{Add, Div, Mul, Neg},
};

#[derive(PartialEq, Clone)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub fn arg(&self) -> f64 {
        if self.im == 0.0 {
            0.0
        } else {
            (self.re / self.im).atan()
        }
    }

    pub fn abs(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn from_polar(abs: f64, arg: f64) -> Complex {
        Complex {
            re: arg.sin() * abs,
            im: arg.cos() * abs,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im == 0.0 {
            self.re.fmt(f)
        } else {
            write!(f, "{}+i{}", self.re, self.im)
        }
    }
}

impl Add for Complex {
    type Output = Complex;
    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        let new_abs = self.abs() * other.abs();
        let new_arg = self.arg() + other.arg();
        Complex::from_polar(new_abs, new_arg)
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let res_abs = self.abs() / other.abs();
        let res_arg = self.arg() - other.arg();
        Complex::from_polar(res_abs, res_arg)
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl From<f64> for Complex {
    fn from(re: f64) -> Complex {
        Complex { re, im: 0.0 }
    }
}

impl Ring for Complex {
    fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }
}

impl AbelianGroup for Complex {
    fn zero() -> Self {
        Complex { re: 0.0, im: 0.0 }
    }
}

impl Field for Complex {
    fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }
    fn inverse(self) -> Self {
        let inv_abs = 1.0 / self.abs();
        let inv_arg = -self.arg();
        Complex::from_polar(inv_abs, inv_arg)
    }
}
