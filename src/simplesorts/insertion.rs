//! Insertion sort algorithm.
//!
//! The `insertion` module contains the simple sorting algorithm "Insertion Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Insertion_sort

extern crate libc;

use std::slice;

/// Simple sort: insertion sort.
///
/// # Details
///
/// Insertion sort is one of the simplest sorting algorithm. Every elements of the list to sort
/// are picked one at a time and inserted into a the sorted list at the right location.
///
/// This function takes an immutable reference vector of any type and returns a vector of `usize`
/// of the same length containing the indices of the initial vector sorted.
///
/// # Scaling
///
/// In the best case scenario, the list is already sorted and the work is simply to append the
/// element at the end of the list, hence a O(N) scaling.
///
/// In the worst case scenario, the list is in reverse order. Hence, every element is compared to
/// all previous elements already sorted until this new one gets inserted at the beginning of the
/// sorted list, thus making this case O(N^2).
///
/// Insertion sort is still useful as its simplicity (and thus small overhead) makes it ideal for
/// small vectors.
///
/// # Optimizations
///
/// Every element of the vector to sort are compared to the first and last element of the sorted
/// vector. This prevents the worst case scenario to happen. The implementation still keeps an
/// O(N^2) scaling though.
///
/// # Notes
///
/// The implementation is 'stable' as it does preserve the relative order of items with
/// equal values.
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// # Examples
///
/// ```
/// let data: Vec<i32> = vec![4, 2, 3, 1];
/// assert_eq!(vec![3, 1, 2, 0], sorting::simplesorts::insertion::sort(&data));
/// ```
///
pub fn sort<T: PartialOrd>(input: &mut [T]) {
    let n = input.len();

    // Start at second element, and insert every elements at the right location
    for i in 1..n {
        // Loop back over the already sorted subvector, and insert element "i" at the
        // proper location.
        for j in 0..i {
            // Element "i" is smaller than "j", insert element "i" before "j" and
            // break the "j" loop.
            if input[j] > input[i] {

                // Move element "i" back in the vector by swaping it with all others until
                // it reaches the proper position (before "j").
                for k in (j..i).rev() {
                    input.swap(k+1, k);
                }

                // Go to the next element "i".
                break;
            }
        }
    }
}



#[no_mangle]
pub extern fn ffi_insertionsort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_insertionsort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_insertionsort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_insertionsort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f64, n as usize)
    };
    sort(&mut to_sort);
}
