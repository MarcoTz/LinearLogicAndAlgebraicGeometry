use super::{group::AbelianGroup, monomial::Monomial, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(PartialEq, Clone)]
pub struct Polynomial<C: Ring, const N: usize> {
    pub monomials: Vec<Monomial<C, N>>,
}

impl<C: Ring, const N: usize> Polynomial<C, N> {
    pub fn eval(&self, x: [C; N]) -> C
    where
        C: Clone,
    {
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
}

impl<C, const N: usize> Ring for Polynomial<C, N>
where
    C: Ring,
    C: Clone,
{
    fn one() -> Polynomial<C, N> {
        Polynomial {
            monomials: vec![Monomial {
                coefficient: C::one(),
                powers: [0; N],
            }],
        }
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

impl<C: Ring, const N: usize> Neg for Polynomial<C, N> {
    type Output = Self;
    fn neg(self) -> Self {
        let monomials = self.monomials.into_iter().map(|mono| -mono).collect();
        Polynomial { monomials }
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

impl<C, const N: usize> Mul for Polynomial<C, N>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Ring,
    C: Clone,
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
