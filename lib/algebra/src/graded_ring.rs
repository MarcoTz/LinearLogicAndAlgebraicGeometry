use super::ring::Ring;

/// Graded rings R = + R_i
/// each R_i is a subgroup
/// elements of R_i are homogeneous with degree i
/// is_homogeneous returns true if a given element is contained in some R_i
/// in this case degree shoud return i, otherwise None
pub trait GradedRing: Ring {
    fn is_homogeneous(&self) -> bool;
    fn degree(&self) -> Option<u32>;
}
