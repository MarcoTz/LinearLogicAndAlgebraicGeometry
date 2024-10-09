use super::{errors::Error, field::Field};

pub struct ProjectivePoint<K: Field> {
    coordinates: Vec<K>,
}

impl<K: Field> ProjectivePoint<K> {
    pub fn new(coordinates: Vec<K>) -> Result<Self, Error> {
        if coordinates.iter().all(|elem| *elem == K::zero()) {
            Err(Error::ProjectiveAllZero)
        } else {
            Ok(ProjectivePoint { coordinates })
        }
    }

    pub fn dim(&self) -> i32 {
        self.coordinates.len() as i32 - 1
    }
}

impl<K: Field + PartialEq> PartialEq for ProjectivePoint<K> {
    fn eq(&self, other: &ProjectivePoint<K>) -> bool {
        if self.dim() != other.dim() {
            return false;
        }
        let mut divisors = vec![];
        for (elem1, elem2) in self.coordinates.iter().zip(other.coordinates.iter()) {
            let new_divisor = elem1.clone().divide(elem2.clone());
            if let Ok(div) = new_divisor {
                divisors.push(div);
            }
        }
        divisors.windows(2).all(|elems| elems[0] == elems[1])
    }
}
