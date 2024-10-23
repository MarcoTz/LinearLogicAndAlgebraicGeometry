use super::ProjectivePoint;
use crate::{errors::Error, field::Field, polynomials::HomogeneousPolynomial};

use std::fmt;

pub struct ProjectiveScheme<K: Field> {
    dim: usize,
    ideal_generators: Vec<HomogeneousPolynomial<K>>,
}

impl<K: Field> ProjectiveScheme<K> {
    pub fn new(
        ideal_generators: Vec<HomogeneousPolynomial<K>>,
    ) -> Result<ProjectiveScheme<K>, Error> {
        HomogeneousPolynomial::check_deg(ideal_generators.as_slice())?;
        let dim = HomogeneousPolynomial::check_dim(ideal_generators.as_slice())?;

        Ok(ProjectiveScheme {
            dim,
            ideal_generators,
        })
    }

    pub fn disjoint_union(self, other: ProjectiveScheme<K>) -> ProjectiveScheme<K>
    where
        K: Clone,
    {
        let mut new_polys = vec![];
        for f in self.ideal_generators.iter() {
            for g in other.ideal_generators.iter() {
                let new_poly = f.clone() * g.clone();
                new_polys.push(new_poly);
            }
        }
        ProjectiveScheme::new(new_polys).unwrap()
    }

    pub fn product(self, other: &ProjectiveScheme<K>) -> ProjectiveScheme<K> {
        todo!()
    }

    pub fn contains(&self, pt: &ProjectivePoint<K>) -> Result<bool, Error>
    where
        K: Clone,
    {
        if pt.dim() != self.dim {
            Err(Error::DimensionMismatch {
                expected: self.dim,
                found: pt.dim(),
            })
        } else {
            Ok(())
        }?;

        for poly in self.ideal_generators.iter() {
            let eval_res = poly.eval(pt.clone().as_arr())?;
            if eval_res != K::zero() {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl<K> fmt::Display for ProjectiveScheme<K>
where
    K: Field,
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ideal_str: Vec<String> = self
            .ideal_generators
            .iter()
            .map(|poly| format!("{}", poly))
            .collect();
        write!(f, "P^{}/<{}>", self.dim, ideal_str.join(", "))
    }
}
