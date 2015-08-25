//! Quicksort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Quicksort".
//!
//! Source: https://en.wikipedia.org/wiki/Quicksort


/// Quicksort
///
/// # Details
///
///
///
/// # Scaling
///
///
///
/// # Optimizations
///
/// None
///
/// # Notes
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// # Examples
///
/// ```
/// let mut data: Vec<i32> = vec![4, 2, 3, 1, 5];
/// sorting::efficientsorts::quick::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: PartialOrd>(array: &mut Vec<T>) {
    unimplemented!();
}
