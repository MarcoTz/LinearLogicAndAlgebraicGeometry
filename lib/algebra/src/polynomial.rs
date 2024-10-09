use super::{group::AbelianGroup, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul},
};

#[derive(Clone, PartialEq)]
pub struct Monomial<C: Ring, const N: usize> {
    pub coefficient: C,
    pub powers: [u32; N],
}

#[derive(PartialEq, Clone)]
pub struct Polynomial<C: Ring, const N: usize> {
    pub monomials: Vec<Monomial<C, N>>,
}

impl<C: Ring, const N: usize> Monomial<C, N> {
    pub fn eval(&self, x: [C; N]) -> C {
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

impl<C: Ring, const N: usize> Polynomial<C, N> {
    pub fn eval(&self, x: [C; N]) -> C {
        let mut res = C::zero();
        for mono in self.monomials.iter() {
            let eval_res = mono.eval(x.clone());
            res = res + eval_res
        }
        res
    }
}

impl<C: Ring, const N: usize> AbelianGroup for Polynomial<C, N> {
    fn zero() -> Polynomial<C, N> {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: C::zero(),
                powers: [0; N],
            }],
        }
    }
    fn neg(self) -> Polynomial<C, N> {
        let neg_monos = self
            .monomials
            .into_iter()
            .map(|mono| Monomial {
                coefficient: mono.coefficient.neg(),
                powers: mono.powers,
            })
            .collect();
        Polynomial {
            monomials: neg_monos,
        }
    }
}

impl<C: Ring, const N: usize> Ring for Polynomial<C, N> {
    fn one() -> Polynomial<C, N> {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: C::one(),
                powers: [0; N],
            }],
        }
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

impl<C, const N: usize> fmt::Display for Polynomial<C, N>
where
    C: fmt::Display,
    C: Ring,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mon_str: Vec<String> = self
            .monomials
            .iter()
            .map(|mon| format!("{}", mon))
            .collect();
        write!(f, "{}", mon_str.join(" + "))
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

impl<C, const N: usize> Add for Polynomial<C, N>
where
    C: Add<Output = C>,
    C: Ring,
{
    type Output = Polynomial<C, N>;
    fn add(self, other: Self) -> Self::Output {
        let mut new_monomials = self.monomials;
        for other_mono in other.monomials.into_iter() {
            match new_monomials
                .iter()
                .enumerate()
                .find(|(_, mono)| mono.powers == other_mono.powers)
            {
                None => {
                    new_monomials.push(other_mono);
                }
                Some((ind, _)) => {
                    let old_mono = new_monomials.remove(ind);
                    let new_poly = old_mono + other_mono;
                    new_monomials.extend(new_poly.monomials);
                }
            }
        }
        Polynomial {
            monomials: new_monomials,
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

impl<C, const N: usize> Mul for Polynomial<C, N>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Ring,
{
    type Output = Polynomial<C, N>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_monomials: Vec<Monomial<C, N>> = vec![];
        for self_mono in self.monomials.iter() {
            for other_mono in other.monomials.iter() {
                let mut new_mono = self_mono.clone() * other_mono.clone();
                if let Some(mono) = new_monomials
                    .iter()
                    .find(|mono| mono.powers == new_mono.powers)
                {
                    new_mono.coefficient = new_mono.coefficient + mono.coefficient.clone();
                }
                new_monomials.push(new_mono);
            }
        }
        Polynomial {
            monomials: new_monomials,
        }
    }
}
