use super::{
    errors::Error, field::Field, monomial::Monomial, polynomial::Polynomial,
    projective_morphism::ProjectiveMorphism, ring::Ring,
};
use std::{fmt, ops::Mul};

#[derive(Clone, PartialEq)]
pub struct HomogeneousPolynomial<R: Ring, const N: usize> {
    deg: u32,
    pub monomials: Vec<Monomial<R, N>>,
}

impl<R: Ring, const N: usize> HomogeneousPolynomial<R, N> {
    pub fn deg(&self) -> u32 {
        self.deg
    }

    pub fn eval(&self, x: [R; N]) -> R
    where
        R: Clone,
    {
        let mut res = R::zero();
        for mono in self.monomials.iter() {
            let eval_res = mono.eval(x.clone());
            res = res + eval_res
        }
        res
    }

    pub fn apply_morphism<const M: usize>(
        self,
        morphism: ProjectiveMorphism<R, M, N>,
    ) -> Result<HomogeneousPolynomial<R, M>, Error>
    where
        R: Field + Clone,
    {
        let mut res = Polynomial { monomials: vec![] };
        for mono in self.monomials {
            let new_poly = mono.compose_morphism(morphism.clone());
            res = res + new_poly;
        }
        res.try_into()
    }
}

impl<R: Ring, const N: usize> From<HomogeneousPolynomial<R, N>> for Polynomial<R, N> {
    fn from(homo: HomogeneousPolynomial<R, N>) -> Polynomial<R, N> {
        Polynomial {
            monomials: homo.monomials,
        }
    }
}

impl<R: Ring, const N: usize> TryFrom<Polynomial<R, N>> for HomogeneousPolynomial<R, N> {
    type Error = Error;
    fn try_from(poly: Polynomial<R, N>) -> Result<HomogeneousPolynomial<R, N>, Error> {
        if poly.monomials.is_empty() {
            return Ok(HomogeneousPolynomial {
                deg: 0,
                monomials: vec![],
            });
        }

        let deg = poly.monomials.first().unwrap().deg();
        for mono in poly.monomials.iter() {
            if mono.deg() != deg {
                return Err(Error::WrongDegree {
                    found: mono.deg(),
                    expected: deg,
                });
            }
        }
        Ok(HomogeneousPolynomial {
            deg,
            monomials: poly.monomials,
        })
    }
}

impl<R, const N: usize> Mul for HomogeneousPolynomial<R, N>
where
    R: Ring,
    R: Clone,
{
    type Output = HomogeneousPolynomial<R, N>;
    fn mul(self, other: HomogeneousPolynomial<R, N>) -> HomogeneousPolynomial<R, N> {
        let new_deg = self.deg + other.deg;
        let mut new_monomials: Vec<Monomial<R, N>> = vec![];
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
        HomogeneousPolynomial {
            deg: new_deg,
            monomials: new_monomials,
        }
    }
}

impl<R, const N: usize> fmt::Display for HomogeneousPolynomial<R, N>
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
