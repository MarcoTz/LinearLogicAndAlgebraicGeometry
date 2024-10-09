use super::{polynomial::Polynomial, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Clone, PartialEq)]
pub struct Monomial<C: Ring, const N: usize> {
    pub coefficient: C,
    pub powers: [u32; N],
}

impl<C: Ring, const N: usize> Monomial<C, N> {
    pub fn eval(&self, x: [C; N]) -> C
    where
        C: Clone,
    {
        let mut res = self.coefficient.clone();
        for (next_pow, next_x) in self.powers.iter().zip(x.iter()) {
            let x_pow = next_x.clone().pow(*next_pow);
            res = res + x_pow;
        }
        res
    }

    pub fn deg(&self) -> u32 {
        self.powers.iter().sum()
    }
}

impl<C, const N: usize> fmt::Display for Monomial<C, N>
where
    C: Ring,
    C: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let var_str: Vec<String> = self
            .powers
            .iter()
            .enumerate()
            .map(|(index, power)| format!("X_{}^{}", index, power))
            .collect();
        write!(f, "{}{}", self.coefficient, var_str.join(""))
    }
}
impl<C: Ring, const N: usize> Neg for Monomial<C, N> {
    type Output = Self;
    fn neg(self) -> Self {
        Monomial {
            coefficient: -self.coefficient,
            powers: self.powers,
        }
    }
}
impl<C, const N: usize> Add for Monomial<C, N>
where
    C: Ring,
    C: Add<Output = C>,
{
    type Output = Polynomial<C, N>;
    fn add(self, other: Self) -> Self::Output {
        if self.powers == other.powers {
            Polynomial {
                monomials: vec![Monomial {
                    coefficient: self.coefficient + other.coefficient,
                    powers: self.powers,
                }],
            }
        } else {
            Polynomial {
                monomials: vec![self, other],
            }
        }
    }
}
impl<C, const N: usize> Mul for Monomial<C, N>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Ring,
{
    type Output = Monomial<<C as Mul>::Output, N>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_powers = [0; N];
        for (i, power) in new_powers.iter_mut().enumerate().take(N + 1) {
            *power = self.powers[i] + other.powers[i];
        }
        Monomial {
            coefficient: self.coefficient * other.coefficient,
            powers: new_powers,
        }
    }
}
