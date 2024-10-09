use super::{
    errors::Error,
    polynomial::{Monomial, Polynomial},
    ring::Ring,
};

pub struct HomogeneousPolynomial<R: Ring, const N: usize> {
    deg: u32,
    pub monomials: Vec<Monomial<R, N>>,
}

impl<R: Ring, const N: usize> HomogeneousPolynomial<R, N> {
    pub fn deg(&self) -> u32 {
        self.deg
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
