//! Quicksort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Quicksort".
//!
//! Source: https://en.wikipedia.org/wiki/Quicksort

extern crate libc;

use std::slice;

/// Quicksort
///
/// # Details
///
/// Quicksort is the defacto standard of sorting algorithms. It recursively sorts a vector in place
/// by finding a pivot and by re-arranging the vector so that all smaller elements are on the
/// left and all larger elements are on the right of that pivot.
///
/// # Scaling
///
/// It's average case complexity is O(N log N). To prevent the worst case complexity of O(N^2) when
/// the vector is already sorted, the median of the first, middle and last elements is taken as the
/// pivot ("median-of-three").
///
/// # Optimizations
///
/// Use median as pivot.
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
pub fn sort<T: PartialOrd>(array: &mut [T]) {
    let n = array.len();

    // NOTE: Insertion could be used for "small" number of elements as an optimization. It is not
    //       used here as to show exactly how quicksort works.

    if n <= 1 {
        // Don't do anything
    } else if n == 2 {
        if array.first() > array.last() {
            array.swap(0, 1);
        }
    } else {
        // Choose the pivot element: Select the median between the first, middle and last element.
        let mut pivot = {
            let mut piv_choices: Vec<usize> = vec![0, n/2, n-1];
            // Find maximum value between the first, middle and last element.
            let mut max_val_i: usize = 0;
            for i in 1..3 {
                if array[piv_choices[max_val_i]] < array[piv_choices[i]] {
                    max_val_i = i;
                }
            }
            // Remove that index from the choice of pivot.
            piv_choices.remove(max_val_i);
            // The pivot will be the maximum of the remaining two (the median of initial
            // piv_choices vector).
            assert_eq!(piv_choices.len(), 2);
            if array[piv_choices[0]] > array[piv_choices[1]] {
                piv_choices[0]
            } else {
                piv_choices[1]
            }
        };

        // Place pivot at i=0 and loop over the remaining of the vector.
        array.swap(pivot, 0);
        pivot = 0;

        for i in 1..n {
            // The loop element is smaller than the pivot. Shuffle things around to place that
            // element before the pivot.
            if array[pivot] > array[i] {
                // First, let's swap the element and the pivot.
                array.swap(pivot, i);
                // Two cases are possible here:
                //     1) The element was the one next to the pivot in the vector. As such, the
                //        new pivot's location is the index "i".
                //     2) The element swaped was further down the vector. If we just swap the pivot
                //        and that element, we are sending the pivot _after_ larger elements,
                //        breaking the ordering! Additionally to the first swap, the pivot (now
                //        being at index "i" after the first swap) is again swapped with the
                //        element that was next to it before (index "pivot+1").
                if i == pivot+1 {
                    pivot = i;
                } else {
                    array.swap(i, pivot+1);
                    pivot = pivot+1;
                }
            }
        }

        // Recursively call the function on slices of the vector.
        sort(&mut array[0..pivot+1]);
        sort(&mut array[pivot+1..n]);
    }
}



#[no_mangle]
pub extern "C" fn ffi_quicksort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_quicksort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_quicksort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f64, n as usize)
    };
    sort(&mut to_sort);
}
