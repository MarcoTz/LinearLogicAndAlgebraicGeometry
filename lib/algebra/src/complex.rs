use std::{
    fmt,
    ops::{Add, Mul},
};

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

impl From<f64> for Complex {
    fn from(re: f64) -> Complex {
        Complex { re, im: 0.0 }
    }
}
