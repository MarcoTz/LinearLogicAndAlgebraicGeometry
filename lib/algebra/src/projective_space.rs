use super::{errors::Error, field::Field};

#[derive(Clone)]
pub struct ProjectivePoint<K: Field, const N: usize> {
    coordinates: [K; N],
}

impl<K: Field, const N: usize> ProjectivePoint<K, N> {
    pub fn new(coordinates: [K; N]) -> Result<Self, Error> {
        if coordinates.iter().all(|elem| *elem == K::zero()) {
            Err(Error::ProjectiveAllZero)
        } else {
            Ok(ProjectivePoint { coordinates })
        }
    }

    pub fn dim(&self) -> i32 {
        self.coordinates.len() as i32 - 1
    }

    pub fn as_arr(self) -> [K; N] {
        self.coordinates
    }
}

impl<K, const N: usize> PartialEq for ProjectivePoint<K, N>
where
    K: Field,
    K: Clone,
{
    fn eq(&self, other: &ProjectivePoint<K, N>) -> bool {
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
