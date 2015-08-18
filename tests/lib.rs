extern crate sorting;

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
fn verify_sorting<T: PartialOrd>(array: &Vec<T>, sorted_indices: &Vec<usize>) {
    let n = array.len();
    assert_eq!(n , sorted_indices.len());

    if !array.is_empty() {
        for i in 0..n-1 {
            assert!(array[sorted_indices[i]] <= array[sorted_indices[i+1]]);
        }
    }
}


/// Call the specified sorting function on an empty vector of specific type.
///
/// # Details
///
/// This function takes as arguments a pointer to a sorting function acting and call
/// it on an empty vector of type T. The call should succeed and the index vector should
/// be empty as well.
///
/// # Panics
///
/// The emptiness of the sorted list is enforced using `assert!()`.
///
/// # Examples
///
/// ```
/// test_empty_vec::<i8>(sorting::simplesorts::insertion::sort);
/// ```
///
fn test_empty_vec<T: PartialOrd>(sorting_fct: fn(&Vec<T>) -> Vec<usize>) {
    let to_sort: Vec<T> = vec![];
    let sorted_indices = sorting_fct(&to_sort);
    assert_eq!(sorted_indices, vec![]);
    assert!(sorted_indices.is_empty());
}



/// Validate against empty vector (i8).
#[test]
fn simple_insertion_empty_vec_i8() {
    test_empty_vec::<i8>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (i32).
#[test]
fn simple_insertion_empty_vec_i16() {
    test_empty_vec::<i16>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (i32).
#[test]
fn simple_insertion_empty_vec_i32() {
    test_empty_vec::<i32>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (i64).
#[test]
fn simple_insertion_empty_vec_i64() {
    test_empty_vec::<i64>(sorting::simplesorts::insertion::sort);
}


/// Validate against empty vector (u8).
#[test]
fn simple_insertion_empty_vec_u8() {
    test_empty_vec::<u8>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (u32).
#[test]
fn simple_insertion_empty_vec_u16() {
    test_empty_vec::<u16>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (u32).
#[test]
fn simple_insertion_empty_vec_u32() {
    test_empty_vec::<u32>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (u64).
#[test]
fn simple_insertion_empty_vec_u64() {
    test_empty_vec::<u64>(sorting::simplesorts::insertion::sort);
}



/// Validate a vector of double precision values.
#[test]
fn simple_insertion_vec_f64() {
    let to_sort: Vec<f64> = vec![6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                 3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];

    let sorted_indices = sorting::simplesorts::insertion::sort(&to_sort);

    verify_sorting(&to_sort, &sorted_indices);

    assert_eq!(sorted_indices, vec![3, 15, 4, 2, 13, 8, 5, 1, 0, 7, 6, 9, 10, 14, 11, 12]);
}
