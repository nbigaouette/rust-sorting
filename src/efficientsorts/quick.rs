//! Quicksort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Quicksort".
//!
//! Source: https://en.wikipedia.org/wiki/Quicksort

extern crate libc;

use std::fmt::Debug;
use std::slice;

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
pub fn sort<T: PartialOrd + Debug>(array: &mut [T]) {
    let n = array.len();

    if n <= 1 {
        // Don't do anything
    } else if n == 2 {
        if array.first() > array.last() {
            array.swap(0, 1);
        }
    } else {
        // Pick a pivot: Pick the middle element by skipping half the length and keeping just one.
        // let it_pivot = array.iter().skip(n / 2).take(1);
        // Take the first element as the pivot.
        // let it_pivot = array.iter().take(1);
        //
        // for it in it_pivot {
        //     println!("  0. it: {:?}", it);
        // }

        // Loop over array, finding smaller elements than the pivot.
        // for it in array.iter().skip(1) {
        //     //println!("  1. it: {:?}", it);
        //     if it < it_pivot {
        //         println!("  1. it: {:?} smaller than pivot!", it);
        //     }
        // }
        // array.swap(0,1);

        // array.iter().enumerate().map(|(i,x)| {});

        // // Take the first element as pivot.
        // let a = array.iter_mut().skip(1).enumerate().fold(
        //     (0, array[0]), |acc, item| {
        //         if acc.1 > item.1 {
        //             // Element is smaller than pivot; swap them!
        //             array.swap(acc.0, item.0);
        //             // Return the accumulator as being the new index of pivot (which is now at
        //             // at the location of the fold()) with the pivot value which shouldn't change.
        //             (item.0, acc.1)
        //         } else {
        //             // Nothing to do as element is larger than pivot. Just return new accumulator.
        //             (acc.0, acc.1)
        //         }
        //     }
        // );

        // Use the first element as the pivot, and loop over the rest of the vector.
        let mut pivot = 0;
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
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_quicksort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}
