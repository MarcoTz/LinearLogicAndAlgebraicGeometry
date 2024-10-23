use super::ProjectivePoint;
use crate::{
    errors::Error,
    field::Field,
    polynomials::{HomogeneousPolynomial, Monomial},
};

#[derive(Clone)]
pub struct ProjectiveMorphism<K: Field> {
    dim_domain: usize,
    dim_codomain: usize,
    coordinate_functions: Vec<HomogeneousPolynomial<K>>,
}

impl<K> ProjectiveMorphism<K>
where
    K: Field,
{
    pub fn new(
        coordinate_functions: Vec<HomogeneousPolynomial<K>>,
    ) -> Result<ProjectiveMorphism<K>, Error> {
        HomogeneousPolynomial::check_deg(coordinate_functions.as_slice())?;
        let dim_domain = HomogeneousPolynomial::check_dim(coordinate_functions.as_slice())?;
        Ok(ProjectiveMorphism {
            dim_domain,
            dim_codomain: coordinate_functions.len(),
            coordinate_functions,
        })
    }

    pub fn eval(&self, pt: ProjectivePoint<K>) -> Result<ProjectivePoint<K>, Error>
    where
        K: Clone,
    {
        if pt.dim() != self.dim_domain {
            Err(Error::DimensionMismatch {
                found: pt.dim(),
                expected: self.dim_domain,
            })
        } else {
            Ok(())
        }?;

        let mut new_coordinates = vec![];
        for fun in self.coordinate_functions.iter() {
            new_coordinates.push(fun.eval(pt.clone().as_arr())?);
        }

        ProjectivePoint::new(new_coordinates)
    }

    pub fn nth_coordinate(&self, n: usize) -> Result<HomogeneousPolynomial<K>, Error>
    where
        K: Clone,
    {
        self.coordinate_functions
            .get(n)
            .cloned()
            .ok_or(Error::DimensionMismatch {
                expected: self.dim_domain,
                found: n,
            })
    }

    /// Segre embedding
    /// P^n x P^m -> P^(n-1)(m-1)-1
    /// [X0:...:Xn] x [Y0:...:Ym] -> [X0Y0:X0Y1:...:X0Ym:...:XnYm]
    pub fn segre_embedding(n: usize, m: usize) -> ProjectiveMorphism<K> {
        let mut coordinate_functions = vec![];
        for x_ind in 0..=n {
            for y_ind in 0..=m {
                let mut powers = vec![0; (n - 1) * (m - 1) - 1];
                powers[x_ind * m + y_ind] = 1;
                let mono = Monomial::new(<K as Field>::one(), powers);
                coordinate_functions.push(mono.into());
            }
        }
        ProjectiveMorphism::new(coordinate_functions).unwrap()
    }
}
