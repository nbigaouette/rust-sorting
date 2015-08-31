//! Heap sort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Heap sort".
//!
//! Source: https://en.wikipedia.org/wiki/Heapsort

extern crate libc;

use std::slice;


/// Heap sort
///
/// # Details
///
///
/// # Scaling
///
/// Heap sort has O(N log N) average case complexity and O(N) space complexity due to the
/// `sift_down()` implementation.
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
/// The design comes from the Wikipedia page on heap sort.
///
/// # Examples
///
/// ```
/// let mut data: Vec<i32> = vec![4, 2, 3, 1, 5];
/// sorting::efficientsorts::heap::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: PartialOrd>(input: &mut [T]) {
    let n = input.len();

    heapify(input);

    let mut end = n - 1;
    while end > 0 {
        input.swap(end, 0);
        end -= 1;
        sift_down(input, 0, end);
    }
}

fn node_parent_id(i: usize) -> usize {
    if i == 0 { 0 }
    else      { (i - 1) / 2 }
}

fn node_child_left(i: usize) -> usize {
    2 * i + 1
}

// fn node_child_right(i: usize) -> usize {
//     2 * i + 2
// }

fn heapify<T: PartialOrd>(input: &mut [T]) {
    let n = input.len();
    let end = n - 1;

    // Last element is at "n-1". Find its parent and start there:
    let mut start: usize = node_parent_id(n-1);

    loop {
        sift_down(input, start, end);
        if start == 0 { break; }
        start -= 1;
    }
}

fn sift_down<T: PartialOrd>(input: &mut [T], start: usize, end: usize) {
    let mut i = start;

    // Starting at the "start" element, loop "down" the vector and swap elements that are
    // not in max-heap order.
    while node_child_left(i) <= end {
        let child = node_child_left(i);
        let mut swap = i;

        if input[swap] < input[child] {
            swap = child;
        }
        if child+1 <= end && input[swap] < input[child+1] {
            swap = child + 1;
        }

        if swap == i {
            return;
        } else {
            input.swap(i, swap);
            i = swap;
        }
    }
}


#[no_mangle]
pub extern "C" fn ffi_heapsort_i8(array_pointer: *const libc::int8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_i16(array_pointer: *const libc::int16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_i32(array_pointer: *const libc::int32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_i64(array_pointer: *const libc::int64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    sort(&mut to_sort);
}


#[no_mangle]
pub extern "C" fn ffi_heapsort_u8(array_pointer: *const libc::uint8_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_u16(array_pointer: *const libc::uint16_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_u32(array_pointer: *const libc::uint32_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_u64(array_pointer: *const libc::uint64_t, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    sort(&mut to_sort);
}

#[no_mangle]
pub extern "C" fn ffi_heapsort_f32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f32, n as usize)
    };
    sort(&mut to_sort);
}
#[no_mangle]
pub extern "C" fn ffi_heapsort_f64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut f64, n as usize)
    };
    sort(&mut to_sort);
}
