extern crate rand;

extern crate sorting;

use rand::Rng;



const TO_SORT_I8:  [i8;  16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_I16: [i16; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_I32: [i32; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_I64: [i64; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_U8:  [u8;  16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_U16: [u16; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_U32: [u32; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const TO_SORT_U64: [u64; 16] = [6,  5,  3,  1,  2, 4, 10, 7, 3, 32, 44, 56, 67, 3, 44, 2];
const SORTED_IND_INT:  [usize; 16] = [3, 4, 15, 2, 8, 13, 5, 1, 0, 7, 6, 9, 10, 14, 11, 12];

const TO_SORT_F32: [f32; 16] = [6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];
const TO_SORT_F64: [f64; 16] = [6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];
// NOTE: The order is different for the floats because of the fractional parts. For example,
//       "2.0" comes before "2.4", while in the integer arrays it was "2" vs "2". Since the
//       algorithms are stable, the floating points vector will get a different sorting sequence.
const SORTED_IND_FLOAT: [usize; 16] = [3, 15, 4, 2, 13, 8, 5, 1, 0, 7, 6, 9, 10, 14, 11, 12];




/// Verify input vector as being sorted.
///
/// # Details
///
/// This function takes as arguments a vector of data that must be verified to be sorted.
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
/// let data: Vec<i32> = vec![1, 2, 3, 4];
/// verify_sorting(&data);
/// ```
///
fn verify_sorted<T: PartialOrd>(array: &Vec<T>) {
    let n = array.len();
    if !array.is_empty() {
        for i in 0..n-1 {
            assert!(array[i] <= array[i+1]);
        }
    }
}


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


/// Call the specified sorting function on an specific vector of floats and validate the output.
///
/// # Details
///
/// This function takes as arguments:
///    1. An immutable vector of floats, to be sorted;
///    2. An immutable vector of indices as the reference to compare with;
///    3. A function pointer to be called on the first vector.
///
/// # Panics
///
/// This calls the function `verify_sorting()` which will assert if the result is not in order.
/// It also compares the resulting indices with expected values using asserts.
///
/// # Examples
///
/// ```
/// let to_sort: Vec<f32> = vec![6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
///                              3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];
///
/// let known_sorted_indices = vec![3, 15, 4, 2, 13, 8, 5, 1, 0, 7, 6, 9, 10, 14, 11, 12];
///
/// test_sort_vec_float::<f32>(&to_sort, &known_sorted_indices,
///                            sorting::simplesorts::insertion::sort);
/// ```
///
fn test_sort_vec_float<T: PartialOrd>(to_sort: &Vec<T>, expected_indices: &Vec<usize>,
                                      sorting_fct: fn(&Vec<T>) -> Vec<usize>) {
    let sorted_indices = sorting_fct(&to_sort);

    verify_sorting(&to_sort, &sorted_indices);

    assert_eq!(sorted_indices.len(), expected_indices.len());

    for i in 0..to_sort.len() {
        assert_eq!(sorted_indices[i], expected_indices[i]);
    }
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

/// Validate against empty vector (isize).
#[test]
fn simple_insertion_empty_vec_isize() {
    test_empty_vec::<isize>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (usize).
#[test]
fn simple_insertion_empty_vec_usize() {
    test_empty_vec::<usize>(sorting::simplesorts::insertion::sort);
}

/// Validate against empty vector (f32).
#[test]
fn simple_insertion_empty_vec_f32() {
    test_empty_vec::<f32>(sorting::simplesorts::insertion::sort);
}
/// Validate against empty vector (f64).
#[test]
fn simple_insertion_empty_vec_f64() {
    test_empty_vec::<f64>(sorting::simplesorts::insertion::sort);
}

/// Validate sorting a vector of single precision values (i8).
#[test]
fn simple_insertion_vec_i8() {
    let to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<i8>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (i16).
#[test]
fn simple_insertion_vec_i16() {
    let to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<i16>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (i32).
#[test]
fn simple_insertion_vec_i32() {
    let to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<i32>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (i64).
#[test]
fn simple_insertion_vec_i64() {
    let to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<i64>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}

/// Validate sorting a vector of single precision values (u8).
#[test]
fn simple_insertion_vec_u8() {
    let to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<u8>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (u16).
#[test]
fn simple_insertion_vec_u16() {
    let to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<u16>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (u32).
#[test]
fn simple_insertion_vec_u32() {
    let to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<u32>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}
/// Validate sorting a vector of single precision values (u64).
#[test]
fn simple_insertion_vec_u64() {
    let to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_INT[..]);

    test_sort_vec_float::<u64>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}


/// Validate sorting a vector of single precision values (f32).
#[test]
fn simple_insertion_vec_f32() {
    let to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_FLOAT[..]);

    test_sort_vec_float::<f32>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}

/// Validate sorting a vector of double precision values (f64).
#[test]
fn simple_insertion_vec_f64() {
    let to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);
    let known_sorted_indices: Vec<usize> = From::from(&SORTED_IND_FLOAT[..]);

    test_sort_vec_float::<f64>(&to_sort, &known_sorted_indices,
                               sorting::simplesorts::insertion::sort);
}


/// Validate sorting of a const vector (i8).
#[test]
fn simple_selection_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i16).
#[test]
fn simple_selection_vec_i16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i32).
#[test]
fn simple_selection_vec_i32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i64).
#[test]
fn simple_selection_vec_i64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u8).
#[test]
fn simple_selection_vec_u8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u16).
#[test]
fn simple_selection_vec_u16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u32).
#[test]
fn simple_selection_vec_u32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u64).
#[test]
fn simple_selection_vec_u64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a const vector (f32).
#[test]
fn simple_selection_vec_f32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (f64).
#[test]
fn simple_selection_vec_f64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (i8).
#[test]
fn simple_selection_rand_vec_i8() {
    let len = 50;
    let mut to_sort: Vec<i8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i8>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i16).
#[test]
fn simple_selection_rand_vec_i16() {
    let len = 50;
    let mut to_sort: Vec<i16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i16>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i32).
#[test]
fn simple_selection_rand_vec_i32() {
    let len = 50;
    let mut to_sort: Vec<i32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i32>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i64).
#[test]
fn simple_selection_rand_vec_i64() {
    let len = 50;
    let mut to_sort: Vec<i64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i64>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u8).
#[test]
fn simple_selection_rand_vec_u8() {
    let len = 50;
    let mut to_sort: Vec<u8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u8>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u16).
#[test]
fn simple_selection_rand_vec_u16() {
    let len = 50;
    let mut to_sort: Vec<u16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u16>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u32).
#[test]
fn simple_selection_rand_vec_u32() {
    let len = 50;
    let mut to_sort: Vec<u32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u32>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u64).
#[test]
fn simple_selection_rand_vec_u64() {
    let len = 50;
    let mut to_sort: Vec<u64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u64>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (f32).
#[test]
fn simple_selection_rand_vec_f32() {
    let len = 50;
    let mut to_sort: Vec<f32> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f32>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (f64).
#[test]
fn simple_selection_rand_vec_f64() {
    let len = 50;
    let mut to_sort: Vec<f64> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f64>>();
    sorting::simplesorts::selection::sort(&mut to_sort);
    verify_sorted(&to_sort);
}
