//! Mergesort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Mergesort".
//!
//! Source: https://en.wikipedia.org/wiki/Merge_sort

extern crate libc;

use std::slice;

/// Mergesort
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
///
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
/// sorting::efficientsorts::merge::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: PartialOrd+Clone>(input: &mut [T]) {
    let n = input.len();

    if n <= 1 {
        // return;
    } else if n == 2 {
        if input.first() > input.last() {
            input.swap(0, 1);
        }
    } else {
        assert!(n > 2);

        // The subvectors of input are now sorted. Merge them into temporary buffer.
        let mut tmp: Vec<T> = Vec::with_capacity(n);

        {
        let n2 = n / 2;
        let (mut split_left, mut split_right) = input.split_at_mut(n2);

        // Recursively call the function on slices of the vector.
        sort(&mut split_left);
        sort(&mut split_right);

        let mut split_left  = split_left.to_vec();
        let mut split_right = split_right.to_vec();

        for _ in 0..n {
            let len_left  = split_left.len();
            let len_right = split_right.len();

            if len_left != 0 && len_right != 0 {
                let take_left: bool = split_left.first() < split_right.first();
                if take_left {
                    tmp.push(split_left.remove(0));
                } else {
                    tmp.push(split_right.remove(0));
                }
            } else if len_left == 0 {
                // Left is now empty
                assert!(len_left  == 0);
                assert!(len_right != 0);
                tmp.push(split_right.remove(0));
            } else {
                // Right is now empty
                assert!(len_left  != 0);
                assert!(len_right == 0);
                tmp.push(split_left.remove(0));
            }
        }
        }

        // Copy content of "tmp" back into "input" vector.
        assert_eq!(tmp.len(), n);
        // input = &mut tmp[..];
        for k in 0..n {
            input[k] = tmp[k].clone();
        }
    }
}



#[no_mangle]
pub extern "C" fn ffi_mergesort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_mergesort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_mergesort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_mergesort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f64, n as usize)
    };
    sort(&mut to_sort);
}
