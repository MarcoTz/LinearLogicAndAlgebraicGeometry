use super::{errors::Error, field::Field};

#[derive(Clone)]
pub struct ProjectivePoint<K: Field> {
    dim: usize,
    coordinates: Vec<K>,
}

impl<K: Field> ProjectivePoint<K> {
    pub fn new(coordinates: Vec<K>) -> Result<Self, Error> {
        if coordinates.iter().all(|elem| *elem == K::zero()) {
            Err(Error::ProjectiveAllZero)
        } else {
            Ok(ProjectivePoint {
                dim: coordinates.len(),
                coordinates,
            })
        }
    }

    pub fn dim(&self) -> usize {
        self.dim
    }

    pub fn as_arr(self) -> Vec<K> {
        self.coordinates
    }
}

impl<K> PartialEq for ProjectivePoint<K>
where
    K: Field + Clone,
{
    fn eq(&self, other: &ProjectivePoint<K>) -> bool {
        if self.dim() != other.dim() {
            return false;
        }
        let mut divisors = vec![];
        for (elem1, elem2) in self.coordinates.iter().zip(other.coordinates.iter()) {
            divisors.push(elem1.clone() / elem2.clone());
        }
        divisors.windows(2).all(|elems| elems[0] == elems[1])
    }
}
