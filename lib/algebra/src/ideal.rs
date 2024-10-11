use super::{group::AbelianSubGroup, ring::Ring};

pub struct Ideal<R: Ring> {
    pub subgroup: AbelianSubGroup<R>,
}
