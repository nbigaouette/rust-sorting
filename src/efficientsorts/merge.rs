//! Mergesort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Mergesort".
//!
//! Source: https://en.wikipedia.org/wiki/Merge_sort

extern crate libc;

use std::slice;
use std::ptr;

/// Mergesort
///
/// # Details
///
/// Merge sort splits a vector into two subvectors and sorts them recursively. Once the two
/// parts are sorted, they are merged together in proper ordering.
///
/// This implementation is "top-down" as it splits the initial vector into two until there is
/// only a single element per subvector. It then moves up the recursion by merging these
/// subvectors.
///
/// # Scaling
///
/// Merge sort is of order O(N log N) as it takes log N steps to recurse into the vectors spliting
/// and N steps per level to merge these subvectors.
///
/// Merge sort cannot merge in-place without shifting elements often in the merge process. To
/// prevent this shifting, a temporary vector is allocated (and freed) at every level of the
/// recursion. This temporary vector has the same size as the vector that is split in two at every
/// level. During the last merging step the allocation is the same size as the initial vector to
/// sort, hence the space complexity is O(N).
///
/// # Optimizations
///
/// An unsafe block is used to copy the content of the temporary vector back into the function
/// argument through `ptr::copy_nonoverlapping()`. Without this, one would have to loop over
/// every elements which would be highly inefficient.
///
/// # Notes
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// Also, the trait `Clone` must also be implemented for `T` to be able to copy the vector elements
/// into the temporary vector.
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
        // Nothing to do
    } else if n == 2 {
        // Manually sort the two elements
        if input.first() > input.last() {
            input.swap(0, 1);
        }
    } else {
        // Perform a merge sort
        assert!(n > 2);

        // Allocate a vector that will hold the sorted "input" values temporarily.
        // This prevents having to move elements inside the "input" vector directly.
        // This makes the implementation O(N) for space complexity.
        let mut tmp: Vec<T> = Vec::with_capacity(n);

        // New code block as to enforce "split_left" and "split_right"'s lifetime, ending the
        // mutable borrow on input. This is required to copy the elements of "tmp" back
        // into "input".
        {
            let n2 = n / 2;
            let (mut split_left, mut split_right) = input.split_at_mut(n2);

            // Recursively call the function on slices of the vector.
            sort(&mut split_left);
            sort(&mut split_right);

            // Keep peekable iterators into left and right slices.
            // NOTE: We need a peekable iterator as we must not iterate over each slice at every
            //       loop iteration.
            let mut iter_left  = split_left.iter().peekable();
            let mut iter_right = split_right.iter().peekable();

            // Loop over the size of the "input" vector. We'll copy the smallest element of
            // either the left or right slice into "tmp".
            for _ in 0..n {
                // If there is still elements in both the left and right slice, take the
                // smallest of the two jump to the next element of the peekable iterator
                // of that particular slice.
                if !iter_left.peek().is_none() && !iter_right.peek().is_none() {
                    // Should we take an element from the left slice?
                    let take_left: bool = iter_left.peek() < iter_right.peek();
                    if take_left {
                        // If so, get the element and push() it at the end of the "tmp" vector.
                        // Use next() to advance that slice's iterator.
                        tmp.push(iter_left.next().cloned().unwrap());
                    } else {
                        // If not, insert the element from the right slice instead.
                        tmp.push(iter_right.next().cloned().unwrap());
                    }
                } else if iter_left.peek().is_none() {
                    // Left slice is now purged: insert into "tmp" elements from the right slice,
                    // advancing the iterator.
                    debug_assert!(iter_left.peek().is_none());
                    debug_assert!(!iter_right.peek().is_none());
                    tmp.push(iter_right.next().cloned().unwrap());
                } else {
                    // Right slice is now purged: insert into "tmp" elements from the left slice,
                    // advancing the iterator.
                    debug_assert!(!iter_left.peek().is_none());
                    debug_assert!(iter_right.peek().is_none());
                    tmp.push(iter_left.next().cloned().unwrap());
                }
            }
        }

        // Copy content of "tmp" back into "input" vector.
        assert_eq!(tmp.len(), n);
        unsafe {
            ptr::copy_nonoverlapping(tmp.as_ptr(), input.as_mut_ptr(), n);
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
