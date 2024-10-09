use super::{field::Field, polynomial::Polynomial, projective_space::ProjectivePoint};
use std::fmt;

pub struct ProjectiveScheme<K: Field> {
    pub ambient_dim: i32,
    pub ideal_generators: Vec<Polynomial<K>>,
}

impl<K: Field> ProjectiveScheme<K> {
    pub fn disjoint_union(self, _other: ProjectiveScheme<K>) -> ProjectiveScheme<K> {
        todo!()
    }

    pub fn contains(&self, _pt: &ProjectivePoint<K>) -> bool {
        for _poly in self.ideal_generators.iter() {
            todo!()
        }
        todo!()
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
        write!(f, "P^{}/<{}>", self.ambient_dim, ideal_str.join(", "))
    }
}
