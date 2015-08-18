extern crate sorting;

use std::fmt::Debug;

/// Verify order of input vector as sorted by indices.
///
/// # Details
///
/// This function takes as arguments a vector of data and a vector representing the indices of the
/// first vector making it sorted.
///
/// The elements of the sorted vector are compared to the next ones to make sure they are less
/// or equal.
///
/// # Panics
///
/// The sorting of the data vector is enforced using `assert!()`.
///
/// # Examples
///
/// ```
/// let data: Vec<i32> = vec![4, 2, 3, 1];
/// let sorted_indices: Vec<usize> = vec![3, 1, 2, 0];
/// verify_sorting(&data, &sorted_indices);
/// ```
///
fn verify_sorting<T: Debug + PartialOrd>(array: &Vec<T>, sorted_indices: &Vec<usize>) {
    let n = array.len();
    assert_eq!(n , sorted_indices.len());

    if !array.is_empty() {
        for i in 0..n-1 {
            println!("array[sorted_indices[i={:?}]]={:?}   array[sorted_indices[i+1={:?}]]={:?}",
                i, array[sorted_indices[i]], i+1, array[sorted_indices[i+1]]);
            assert!(array[sorted_indices[i]] <= array[sorted_indices[i+1]]);
        }
    }
}


/// Validate against empty vector.
#[test]
fn simple_insertion_empty() {
    let to_sort: Vec<i32> = vec![];

    let sorted_indices = sorting::simplesorts::insertion::sort(&to_sort);

    verify_sorting(&to_sort, &sorted_indices);

    assert_eq!(sorted_indices, vec![]);
}

/// Validate a vector of double precision values.
#[test]
fn simple_insertion_multiple_sizes_f64() {
    let to_sort: Vec<f64> = vec![6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                 3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];

    let sorted_indices = sorting::simplesorts::insertion::sort(&to_sort);

    verify_sorting(&to_sort, &sorted_indices);
}
