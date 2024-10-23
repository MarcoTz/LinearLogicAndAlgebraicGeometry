use super::{HomogeneousPolynomial, Polynomial};
use crate::{errors::Error, field::Field, projective::ProjectiveMorphism, ring::Ring};
use std::{
    fmt,
    ops::{Add, Mul, Neg},
};

#[derive(Clone, PartialEq)]
pub struct Monomial<R: Ring> {
    dim: usize,
    pub coefficient: R,
    powers: Vec<usize>,
}

impl<R: Ring> Monomial<R> {
    pub fn new(coefficient: R, powers: Vec<usize>) -> Monomial<R> {
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

    pub fn eval(&self, x: Vec<R>) -> Result<R, Error>
    where
        R: Clone,
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

    pub fn compose_monomial(self, other: Monomial<R>) -> Monomial<R>
    where
        R: Clone,
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
        morphism: ProjectiveMorphism<R>, //, M, N>,
    ) -> Result<Polynomial<R>, Error>
    //M
    where
        R: Field + Clone,
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

    pub fn check_deg(monos: &[Monomial<R>]) -> Result<(), Error> {
        let deg = match monos.first() {
            None => return Ok(()),
            Some(mono) => mono.deg(),
        };
        match monos.iter().find(|mono| mono.deg() != deg) {
            None => Ok(()),
            Some(mono) => Err(Error::WrongDegree {
                expected: deg,
                found: mono.deg(),
            }),
        }
    }
}

impl<R: Ring> From<Monomial<R>> for Polynomial<R> {
    fn from(mono: Monomial<R>) -> Polynomial<R> {
        Polynomial::new(vec![mono])
    }
}

impl<R: Ring> From<Monomial<R>> for HomogeneousPolynomial<R> {
    fn from(mono: Monomial<R>) -> HomogeneousPolynomial<R> {
        HomogeneousPolynomial::new(vec![mono]).unwrap()
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
