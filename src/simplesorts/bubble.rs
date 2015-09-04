//! Bubble sort algorithm.
//!
//! The `bubble` module contains the simple sorting algorithm "Bubble Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Bubble_sort

extern crate libc;

use std::slice;

/// Simple sort: bubble sort.
///
/// # Details
///
/// Bubble sort will "bubble" elements from a lower poisition to their sorted position. It is
/// a *bad* sorting algorithm but extremely simple to implement.
///
/// # Scaling
///
/// Two nested loops are the basis of bubble sort. The first one indicates the number of elements
/// to skip from the end in the second loop. The second loop iterates over elements, starting at
/// the begining of the vector and ending at the index of the outer loop.
///
/// Because of these two loops, the vector is completely visited for every elements, hence the
/// algorithm's O(N^2) scaling.
///
/// If the vector is already sorted, no swap will take place and this can be detected at the end
/// of the first inner loop. As such, the best case scenario for bubble sort is O(N) when the
/// vector is already sorted.
///
/// # Optimizations
///
/// Detect if no swap occurred during the inner loop. In that case, break the outer loop. This
/// makes the function returns as soon as the vector is sorted.
///
/// If the vector is initially sorted, the best case scenario of O(N) scaling is then achieved.
///
/// # Notes
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// Just don't use this sorting function. It's purely academic.
///
/// # Examples
///
/// ```
/// let mut data: Vec<i32> = vec![4, 2, 3, 1, 5];
/// sorting::simplesorts::bubble::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: PartialOrd>(input: &mut [T]) {
    let n = input.len();

    // External loop indicates the number of elements to skip at end of vector
    for i in 0..n {

        let mut swap_occured: bool = false;

        // Internal loop performs the comparison between elements and the next one.
        // Note that the end of iteration is the total number of elements minus "i" minus one. The
        // minus one is important as we compare element "j" with the next one "j+1".
        for j in 0..n-i-1 {
            // If element is larger than the next one, swap them.
            if input[j] > input[j+1] {
                input.swap(j,j+1);
                swap_occured = true;
            }
        }

        // Optimization: if no swap occurred during this inner loop, the vector is sorted.
        if !swap_occured {
            break;
        }
    }
}



#[no_mangle]
pub extern fn ffi_bubblesort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_bubblesort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_bubblesort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_bubblesort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f64, n as usize)
    };
    sort(&mut to_sort);
}
