use super::{field::Field, polynomial::Polynomial, projective_space::ProjectivePoint};
use std::fmt;

pub struct ProjectiveScheme<K: Field, const N: usize> {
    pub ideal_generators: Vec<Polynomial<K, N>>,
}

impl<K: Field, const N: usize> ProjectiveScheme<K, N> {
    pub fn disjoint_union(self, other: ProjectiveScheme<K, N>) -> ProjectiveScheme<K, N> {
        let mut new_polys = vec![];
        for f in self.ideal_generators.iter() {
            for g in other.ideal_generators.iter() {
                let new_poly = f.clone() * g.clone();
                new_polys.push(new_poly);
            }
        }
        ProjectiveScheme {
            ideal_generators: new_polys,
        }
    }

    pub fn contains(&self, pt: &ProjectivePoint<K, N>) -> bool {
        for poly in self.ideal_generators.iter() {
            if poly.eval(pt.clone().as_arr()) != K::zero() {
                return false;
            }
        }
        true
    }
}

impl<K, const N: usize> fmt::Display for ProjectiveScheme<K, N>
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
        write!(f, "P^{}/<{}>", N, ideal_str.join(", "))
    }
}
