use super::{
    errors::Error, field::Field, homogeneous_polynomial::HomogeneousPolynomial,
    projective_space::ProjectivePoint,
};
use std::array::from_fn;

#[derive(Clone)]
pub struct ProjectiveMorphism<K: Field, const N: usize, const M: usize> {
    pub coordinate_functions: [HomogeneousPolynomial<K, N>; M],
}

impl<K, const N: usize, const M: usize> ProjectiveMorphism<K, N, M>
where
    K: Field,
{
    pub fn eval(&self, pt: ProjectivePoint<K, N>) -> Result<ProjectivePoint<K, M>, Error>
    where
        K: Clone,
    {
        let mut new_coordinates = from_fn(|_| K::zero());
        for i in 1..M {
            new_coordinates[i] = self.coordinate_functions[i].eval(pt.clone().as_arr());
        }
        ProjectivePoint::new(new_coordinates)
    }
}
