use super::{homogeneous_polynomial::HomogeneousPolynomial, monomial::Monomial};
use crate::{errors::Error, graded_ring::GradedRing, group::AbelianGroup, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(PartialEq, Clone)]
pub struct Polynomial<C: Ring> {
    dim: usize,
    monomials: Vec<Monomial<C>>,
}

impl<C: Ring> Polynomial<C> {
    pub fn new(monomials: Vec<Monomial<C>>) -> Polynomial<C> {
        Polynomial {
            dim: monomials.len(),
            monomials,
        }
    }

    pub fn monomials(&self) -> Vec<Monomial<C>>
    where
        C: Clone,
    {
        self.monomials.clone()
    }

    pub fn eval(&self, x: Vec<C>) -> Result<C, Error>
    where
        C: Clone,
    {
        if self.dim != x.len() {
            Err(Error::DimensionMismatch {
                found: x.len(),
                expected: self.dim,
            })
        } else {
            Ok(())
        }?;

        let mut res = C::zero();
        for mono in self.monomials.iter() {
            let eval_res = mono.eval(x.clone())?;
            res = res + eval_res
        }
        Ok(res)
    }
}

impl<C: Ring> AbelianGroup for Polynomial<C> {
    fn zero() -> Polynomial<C> {
        Polynomial::new(vec![Monomial::new(C::zero(), vec![])])
    }
}

impl<C> Ring for Polynomial<C>
where
    C: Ring + Clone,
{
    fn one() -> Polynomial<C> {
        Polynomial::new(vec![Monomial::new(C::one(), vec![])])
    }
}

impl<C> fmt::Display for Polynomial<C>
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

impl<C: Ring> Neg for Polynomial<C> {
    type Output = Self;
    fn neg(self) -> Self {
        let monomials = self.monomials.into_iter().map(|mono| -mono).collect();
        Polynomial::new(monomials)
    }
}

impl<C> Add for Polynomial<C>
where
    C: Add<Output = C>,
    C: Ring,
{
    type Output = Polynomial<C>;
    fn add(self, other: Self) -> Self::Output {
        let mut new_monomials = self.monomials;
        for other_mono in other.monomials.into_iter() {
            match new_monomials
                .iter()
                .enumerate()
                .find(|(_, mono)| mono.powers() == other_mono.powers())
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
        Polynomial::new(new_monomials)
    }
}

impl<C> Mul for Polynomial<C>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Ring + Clone,
{
    type Output = Polynomial<C>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_monomials: Vec<Monomial<C>> = vec![];
        for self_mono in self.monomials.iter() {
            for other_mono in other.monomials.iter() {
                let mut new_mono = self_mono.clone() * other_mono.clone();
                if let Some(mono) = new_monomials
                    .iter()
                    .find(|mono| mono.powers() == new_mono.powers())
                {
                    new_mono.coefficient = new_mono.coefficient + mono.coefficient.clone();
                }
                new_monomials.push(new_mono);
            }
        }
        Polynomial::new(new_monomials)
    }
}

impl<C> GradedRing for Polynomial<C>
where
    C: Ring + Clone,
{
    fn is_homogeneous(&self) -> bool {
        <Polynomial<C> as TryInto<HomogeneousPolynomial<C>>>::try_into(self.to_owned()).is_ok()
    }

    fn degree(&self) -> Option<usize> {
        <Polynomial<C> as TryInto<HomogeneousPolynomial<C>>>::try_into(self.to_owned())
            .map(|f| Some(f.deg()))
            .unwrap_or_default()
    }
}
