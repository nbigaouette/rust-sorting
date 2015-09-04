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

const TO_SORT_F32: [f32; 16] = [6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];
const TO_SORT_F64: [f64; 16] = [6.0,   5.0,  3.0,  1.0,  2.4, 4.0, 10.0, 7.0,
                                3.42, 32.2, 44.2, 56.3, 67.9, 3.2, 44.2, 2.0];



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
fn verify_sorted<T: PartialOrd>(array: &[T]) {
    let n = array.len();
    assert!(array.windows(2).all(|w| w[0] <= w[1]));
    if !array.is_empty() {
        for i in 0..n-1 {
            assert!(array[i] <= array[i+1]);
        }
    }
}


fn test_sort_vec<T: PartialOrd>(to_sort: &mut [T], sorting_fct: fn(&mut [T])) {
    sorting_fct(to_sort);
    verify_sorted(to_sort);
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
fn test_empty_vec<T: PartialOrd>(sorting_fct: fn(&mut [T])) {
    let mut to_sort: Vec<T> = vec![];
    sorting_fct(&mut to_sort);
    assert_eq!(to_sort.len(), 0);
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

/// Validate sorting of a const vector (i8).
#[test]
fn simple_insertion_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);

    test_sort_vec::<i8>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (i16).
#[test]
fn simple_insertion_vec_i16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);

    test_sort_vec::<i16>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (i32).
#[test]
fn simple_insertion_vec_i32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);

    test_sort_vec::<i32>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (i64).
#[test]
fn simple_insertion_vec_i64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);

    test_sort_vec::<i64>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (u8).
#[test]
fn simple_insertion_vec_u8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);

    test_sort_vec::<u8>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (u16).
#[test]
fn simple_insertion_vec_u16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);

    test_sort_vec::<u16>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (u32).
#[test]
fn simple_insertion_vec_u32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);

    test_sort_vec::<u32>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting of a const vector (u64).
#[test]
fn simple_insertion_vec_u64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);

    test_sort_vec::<u64>(&mut to_sort, sorting::simplesorts::insertion::sort);
}


/// Validate sorting a vector of single precision values (f32).
#[test]
fn simple_insertion_vec_f32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);

    test_sort_vec::<f32>(&mut to_sort, sorting::simplesorts::insertion::sort);
}

/// Validate sorting a vector of double precision values (f64).
#[test]
fn simple_insertion_vec_f64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);

    test_sort_vec::<f64>(&mut to_sort, sorting::simplesorts::insertion::sort);
}



/// Validate against empty vector (isize).
#[test]
fn simple_selection_empty_vec_isize() {
    test_empty_vec::<isize>(sorting::simplesorts::selection::sort);
}

/// Validate against empty vector (isize).
#[test]
fn simple_selection_empty_vec_usize() {
    test_empty_vec::<usize>(sorting::simplesorts::selection::sort);
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









/// Validate against empty vector (isize).
#[test]
fn simple_bubble_empty_vec_isize() {
    test_empty_vec::<isize>(sorting::simplesorts::bubble::sort);
}

/// Validate against empty vector (isize).
#[test]
fn simple_bubble_empty_vec_usize() {
    test_empty_vec::<usize>(sorting::simplesorts::bubble::sort);
}

/// Validate sorting of a const vector (i8).
#[test]
fn simple_bubble_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i16).
#[test]
fn simple_bubble_vec_i16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i32).
#[test]
fn simple_bubble_vec_i32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i64).
#[test]
fn simple_bubble_vec_i64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u8).
#[test]
fn simple_bubble_vec_u8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u16).
#[test]
fn simple_bubble_vec_u16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u32).
#[test]
fn simple_bubble_vec_u32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u64).
#[test]
fn simple_bubble_vec_u64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a const vector (f32).
#[test]
fn simple_bubble_vec_f32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (f64).
#[test]
fn simple_bubble_vec_f64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (i8).
#[test]
fn simple_bubble_rand_vec_i8() {
    let len = 50;
    let mut to_sort: Vec<i8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i8>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i16).
#[test]
fn simple_bubble_rand_vec_i16() {
    let len = 50;
    let mut to_sort: Vec<i16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i16>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i32).
#[test]
fn simple_bubble_rand_vec_i32() {
    let len = 50;
    let mut to_sort: Vec<i32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i32>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i64).
#[test]
fn simple_bubble_rand_vec_i64() {
    let len = 50;
    let mut to_sort: Vec<i64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i64>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u8).
#[test]
fn simple_bubble_rand_vec_u8() {
    let len = 50;
    let mut to_sort: Vec<u8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u8>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u16).
#[test]
fn simple_bubble_rand_vec_u16() {
    let len = 50;
    let mut to_sort: Vec<u16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u16>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u32).
#[test]
fn simple_bubble_rand_vec_u32() {
    let len = 50;
    let mut to_sort: Vec<u32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u32>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u64).
#[test]
fn simple_bubble_rand_vec_u64() {
    let len = 50;
    let mut to_sort: Vec<u64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u64>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (f32).
#[test]
fn simple_bubble_rand_vec_f32() {
    let len = 50;
    let mut to_sort: Vec<f32> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f32>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (f64).
#[test]
fn simple_bubble_rand_vec_f64() {
    let len = 50;
    let mut to_sort: Vec<f64> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f64>>();
    sorting::simplesorts::bubble::sort(&mut to_sort);
    verify_sorted(&to_sort);
}











#[test]
fn efficient_quicksort_rand_vec_f64() {
    // let lens: Vec<usize> = vec![0, 1, 2, 3, 50];
    let lens: Vec<usize> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 50, 100];
    for len in lens {
        let mut to_sort: Vec<f64> = vec![0.; len];
        to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f64>>();
        sorting::efficientsorts::quick::sort(&mut to_sort);
        verify_sorted(&to_sort);
    }
}


/// Validate sorting of a const vector (i8).
#[test]
fn efficient_quicksort_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    sorting::efficientsorts::quick::sort(&mut to_sort);
    verify_sorted(&to_sort);
}









/// Validate against empty vector (isize).
#[test]
fn efficient_merge_empty_vec_isize() {
    test_empty_vec::<isize>(sorting::efficientsorts::merge::sort);
}

/// Validate against empty vector (isize).
#[test]
fn efficient_merge_empty_vec_usize() {
    test_empty_vec::<usize>(sorting::efficientsorts::merge::sort);
}

/// Validate sorting of a const vector (i8).
#[test]
fn simple_merge_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i16).
#[test]
fn simple_merge_vec_i16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i32).
#[test]
fn simple_merge_vec_i32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i64).
#[test]
fn simple_merge_vec_i64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u8).
#[test]
fn simple_merge_vec_u8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u16).
#[test]
fn simple_merge_vec_u16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u32).
#[test]
fn simple_merge_vec_u32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u64).
#[test]
fn simple_merge_vec_u64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a const vector (f32).
#[test]
fn simple_merge_vec_f32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (f64).
#[test]
fn simple_merge_vec_f64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (i8).
#[test]
fn simple_merge_rand_vec_i8() {
    let len = 50;
    let mut to_sort: Vec<i8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i8>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i16).
#[test]
fn simple_merge_rand_vec_i16() {
    let len = 50;
    let mut to_sort: Vec<i16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i16>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i32).
#[test]
fn simple_merge_rand_vec_i32() {
    let len = 50;
    let mut to_sort: Vec<i32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i32>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i64).
#[test]
fn simple_merge_rand_vec_i64() {
    let len = 50;
    let mut to_sort: Vec<i64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i64>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u8).
#[test]
fn simple_merge_rand_vec_u8() {
    let len = 50;
    let mut to_sort: Vec<u8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u8>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u16).
#[test]
fn simple_merge_rand_vec_u16() {
    let len = 50;
    let mut to_sort: Vec<u16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u16>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u32).
#[test]
fn simple_merge_rand_vec_u32() {
    let len = 50;
    let mut to_sort: Vec<u32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u32>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u64).
#[test]
fn simple_merge_rand_vec_u64() {
    let len = 50;
    let mut to_sort: Vec<u64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u64>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (f32).
#[test]
fn simple_merge_rand_vec_f32() {
    let len = 50;
    let mut to_sort: Vec<f32> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f32>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (f64).
#[test]
fn simple_merge_rand_vec_f64() {
    let len = 50;
    let mut to_sort: Vec<f64> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f64>>();
    sorting::efficientsorts::merge::sort(&mut to_sort);
    verify_sorted(&to_sort);
}










/// Validate against empty vector (isize).
#[test]
fn efficient_heap_empty_vec_isize() {
    test_empty_vec::<isize>(sorting::efficientsorts::heap::sort);
}

/// Validate against empty vector (isize).
#[test]
fn efficient_heap_empty_vec_usize() {
    test_empty_vec::<usize>(sorting::efficientsorts::heap::sort);
}

/// Validate sorting of a const vector (i8).
#[test]
fn efficient_heap_vec_i8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I8[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i16).
#[test]
fn efficient_heap_vec_i16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I16[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i32).
#[test]
fn efficient_heap_vec_i32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I32[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (i64).
#[test]
fn efficient_heap_vec_i64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_I64[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u8).
#[test]
fn efficient_heap_vec_u8() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U8[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u16).
#[test]
fn efficient_heap_vec_u16() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U16[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u32).
#[test]
fn efficient_heap_vec_u32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U32[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (u64).
#[test]
fn efficient_heap_vec_u64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_U64[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a const vector (f32).
#[test]
fn efficient_heap_vec_f32() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F32[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a const vector (f64).
#[test]
fn efficient_heap_vec_f64() {
    let mut to_sort: Vec<_> = From::from(&TO_SORT_F64[..]);
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (i8).
#[test]
fn efficient_heap_rand_vec_i8() {
    let len = 50;
    let mut to_sort: Vec<i8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i8>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i16).
#[test]
fn efficient_heap_rand_vec_i16() {
    let len = 50;
    let mut to_sort: Vec<i16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i16>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i32).
#[test]
fn efficient_heap_rand_vec_i32() {
    let len = 50;
    let mut to_sort: Vec<i32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i32>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (i64).
#[test]
fn efficient_heap_rand_vec_i64() {
    let len = 50;
    let mut to_sort: Vec<i64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49, 51)).collect::<Vec<i64>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u8).
#[test]
fn efficient_heap_rand_vec_u8() {
    let len = 50;
    let mut to_sort: Vec<u8> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u8>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u16).
#[test]
fn efficient_heap_rand_vec_u16() {
    let len = 50;
    let mut to_sort: Vec<u16> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u16>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u32).
#[test]
fn efficient_heap_rand_vec_u32() {
    let len = 50;
    let mut to_sort: Vec<u32> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u32>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (u64).
#[test]
fn efficient_heap_rand_vec_u64() {
    let len = 50;
    let mut to_sort: Vec<u64> = vec![0; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(1, 101)).collect::<Vec<u64>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}


/// Validate sorting of a random vector (f32).
#[test]
fn efficient_heap_rand_vec_f32() {
    let len = 50;
    let mut to_sort: Vec<f32> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f32>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}

/// Validate sorting of a random vector (f64).
#[test]
fn efficient_heap_rand_vec_f64() {
    let len = 50;
    let mut to_sort: Vec<f64> = vec![0.; len];
    to_sort = to_sort.iter().map(|_| rand::thread_rng().gen_range(-49.0, 51.0)).collect::<Vec<f64>>();
    sorting::efficientsorts::heap::sort(&mut to_sort);
    verify_sorted(&to_sort);
}
