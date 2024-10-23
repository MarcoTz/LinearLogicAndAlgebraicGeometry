use super::{
    errors::Error, field::Field, polynomials::HomogeneousPolynomial,
    projective_space::ProjectivePoint,
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
}
