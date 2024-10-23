use super::polynomial::Polynomial;
use crate::{errors::Error, field::Field, projective_morphism::ProjectiveMorphism, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Clone, PartialEq)]
pub struct Monomial<C: Ring> {
    dim: usize,
    pub coefficient: C,
    powers: Vec<usize>,
}

impl<C: Ring> Monomial<C> {
    pub fn new(coefficient: C, powers: Vec<usize>) -> Monomial<C> {
        Monomial {
            dim: powers.len(),
            coefficient,
            powers,
        }
    }

    pub fn powers(&self) -> Vec<usize> {
        self.powers.clone()
    }

    pub fn dim(&self) -> usize {
        self.dim
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

        let mut res = self.coefficient.clone();
        for (next_pow, next_x) in self.powers.iter().zip(x.iter()) {
            let x_pow = next_x.clone().pow(*next_pow);
            res = res + x_pow;
        }
        Ok(res)
    }

    pub fn deg(&self) -> usize {
        self.powers.iter().sum()
    }

    pub fn compose_monomial(self, other: Monomial<C>) -> Monomial<C>
    where
        C: Clone,
    {
        let new_powers = other
            .powers
            .into_iter()
            .map(|pow| pow * self.deg())
            .collect();
        Monomial::new(
            self.coefficient * other.coefficient.pow(self.dim),
            new_powers,
        )
    }

    pub fn compose_morphism(
        self,
        morphism: ProjectiveMorphism<C>, //, M, N>,
    ) -> Result<Polynomial<C>, Error>
    //M
    where
        C: Field + Clone,
    {
        let mut res = Polynomial::new(vec![]);
        for j in 0..self.dim {
            let nth_coordinate = morphism.nth_coordinate(j)?;
            for mono in nth_coordinate.monomials() {
                res = res + self.clone().compose_monomial(mono.clone()).into();
            }
        }
        Ok(res)
    }
}

impl<C: Ring> From<Monomial<C>> for Polynomial<C> {
    fn from(mono: Monomial<C>) -> Polynomial<C> {
        Polynomial::new(vec![mono])
    }
}

impl<C> fmt::Display for Monomial<C>
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
impl<C: Ring> Neg for Monomial<C> {
    type Output = Self;
    fn neg(self) -> Self {
        Monomial::new(-self.coefficient, self.powers)
    }
}

impl<C> Add for Monomial<C>
where
    C: Ring,
    C: Add<Output = C>,
{
    type Output = Polynomial<C>;
    fn add(self, other: Self) -> Self::Output {
        if self.powers == other.powers {
            Polynomial::new(vec![Monomial::new(
                self.coefficient + other.coefficient,
                self.powers,
            )])
        } else {
            Polynomial::new(vec![self, other])
        }
    }
}

impl<C> Mul for Monomial<C>
where
    C: Add<Output = C>,
    C: Mul<Output = C>,
    C: Ring,
{
    type Output = Monomial<<C as Mul>::Output>;
    fn mul(self, other: Self) -> Self::Output {
        let mut new_powers = vec![];
        for i in 0..self.dim {
            new_powers.push(self.powers[i] + other.powers[i]);
        }
        Monomial::new(self.coefficient * other.coefficient, new_powers)
    }
}
