use super::{monomial::Monomial, polynomial::Polynomial};
use crate::{errors::Error, field::Field, projective::ProjectiveMorphism, ring::Ring};
use std::{fmt, ops::Mul};

#[derive(Clone, PartialEq)]
pub struct HomogeneousPolynomial<R: Ring> {
    dim: usize,
    deg: usize,
    monomials: Vec<Monomial<R>>,
}

impl<R: Ring> HomogeneousPolynomial<R> {
    pub fn new(monomials: Vec<Monomial<R>>) -> Result<HomogeneousPolynomial<R>, Error> {
        let mono1 = monomials.first();
        let deg = match mono1 {
            None => {
                return Ok(HomogeneousPolynomial {
                    dim: 0,
                    deg: 0,
                    monomials: vec![],
                })
            }
            Some(mono) => mono.deg(),
        };
        Monomial::check_deg(monomials.as_slice())?;

        Ok(HomogeneousPolynomial {
            dim: monomials.iter().map(|mono| mono.dim()).max().unwrap(),
            deg,
            monomials,
        })
    }

    pub fn deg(&self) -> usize {
        self.deg
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn monomials(&self) -> Vec<Monomial<R>>
    where
        R: Clone,
    {
        self.monomials.clone()
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

        let mut res = R::zero();
        for mono in self.monomials.iter() {
            let eval_res = mono.eval(x.clone())?;
            res = res + eval_res
        }
        Ok(res)
    }

    pub fn compose_morphism(
        self,
        morphism: &ProjectiveMorphism<R>,
    ) -> Result<HomogeneousPolynomial<R>, Error>
    where
        R: Field + Clone,
    {
        let mut res = Polynomial::new(vec![]);
        for mono in self.monomials {
            let new_poly = mono.compose_morphism(morphism.clone())?;
            res = res + new_poly;
        }
        res.try_into()
    }

    pub fn product(self, other: HomogeneousPolynomial<R>) -> HomogeneousPolynomial<R>
    where
        R: Clone,
    {
        let new_other: Vec<Monomial<R>> = other
            .monomials()
            .into_iter()
            .map(|mono| mono.shift_powers(self.dim))
            .collect();
        let mut new_monomials = self.monomials;
        new_monomials.extend(new_other);
        HomogeneousPolynomial::new(new_monomials).unwrap()
    }

    pub fn check_deg(polys: &[Self]) -> Result<usize, Error> {
        let deg = match polys.first() {
            None => return Ok(0),
            Some(poly) => poly.deg(),
        };
        match polys.iter().find(|poly| poly.deg != deg) {
            None => Ok(deg),
            Some(poly) => Err(Error::WrongDegree {
                expected: deg,
                found: poly.deg(),
            }),
        }
    }

    pub fn check_dim(polys: &[Self]) -> Result<usize, Error> {
        let dim = match polys.first() {
            None => return Ok(0),
            Some(poly) => poly.dim(),
        };
        match polys.iter().find(|poly| poly.dim() != dim) {
            None => Ok(dim),
            Some(poly) => Err(Error::DimensionMismatch {
                found: poly.dim(),
                expected: dim,
            }),
        }
    }
}

impl<R: Ring> From<HomogeneousPolynomial<R>> for Polynomial<R> {
    fn from(homo: HomogeneousPolynomial<R>) -> Polynomial<R> {
        Polynomial::new(homo.monomials)
    }
}

impl<R: Ring + Clone> TryFrom<Polynomial<R>> for HomogeneousPolynomial<R> {
    type Error = Error;
    fn try_from(poly: Polynomial<R>) -> Result<HomogeneousPolynomial<R>, Error> {
        HomogeneousPolynomial::new(poly.monomials())
    }
}

impl<R> Mul for HomogeneousPolynomial<R>
where
    R: Ring,
    R: Clone,
{
    type Output = HomogeneousPolynomial<R>;
    fn mul(self, other: HomogeneousPolynomial<R>) -> HomogeneousPolynomial<R> {
        let mut new_monomials: Vec<Monomial<R>> = vec![];
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
        HomogeneousPolynomial::new(new_monomials).unwrap()
    }
}

impl<R> fmt::Display for HomogeneousPolynomial<R>
where
    R: fmt::Display,
    R: Ring,
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
