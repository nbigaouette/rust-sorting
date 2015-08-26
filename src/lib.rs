//! Sorting algorithms implementations in Rust (1.2).
//!
//! # Goal
//!
//! The goal of this project is purely educational and contains two aspects.
//!
//! First is to learn different canonical sorting algorithms and their implementations.
//! Starting on Wikipedia's
//! [sorting algorithms page](https://en.wikipedia.org/wiki/Sorting_algorithm)
//! I read the summary of some of the algorithms described here and tried to implement them without
//! looking at the details. I did not research any optimization on them and as such, there might be
//! faster (but more complicated) ways of implementing them. I did put some though in the
//! implementations as to have efficient and clean implementations without wasting memory.
//!
//! My second goal is to learn Rust, a new language which I find quite promising. Created at
//! Mozilla and developed in the open by a wonderful community, Rust has an initial goal of
//! creating a language to replace C++ as the language of choice for Firefox's rendering engine.
//!
//! Some of the goals of Rust are:
//!
//! * Memory safety without garbage collection
//! * Concurrency without data races
//! * Abstraction without overhead
//! * Stability without stagnation
//!
//! For more information on Rust, see:
//!
//! * Rust homepage: https://www.rust-lang.org/
//! * Wikipedia: https://en.wikipedia.org/wiki/Rust_%28programming_language%29
//!
//! The code should be well commented and simple to follow and/or expand. Unit tests are used to
//! validate every implementations.
//!
//! This module is greatly inspired by my C++ templated sorting library located here:
//! https://github.com/nbigaouette/sorting.
//!
//! # Algorithms
//!
//! Sorting functions are encapsulated into modules. Implemented algorithms are:
//!
//! ## Simple sorts
//!
//! * [Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort)
//! * [Selection sort](https://en.wikipedia.org/wiki/Selection_sort)
//!
//! ## Efficient sorts
//!
//! * [Quicksort](https://en.wikipedia.org/wiki/Quicksort)
//!
//!
//! # Usage
//!
//!
//! # Notes
//!
//!
//! # Testing and validation
//!
//!
//! # License
//! This code is distributed under the terms of the BSD 3-clause "New" or "Revised" License
//! and is Copyright 2014 Nicolas Bigaouette.
//!
//!

pub mod simplesorts;
pub mod efficientsorts;


// Expose Rust's sort() method as if it was implemented here.
// Useful for benchmarking and comparison.

extern crate libc;

use std::slice;


#[no_mangle]
pub extern "C" fn ffi_sort_i8(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i8, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_i16(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i16, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_i32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i32, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_i64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut i64, n as usize)
    };
    to_sort.sort();
}


#[no_mangle]
pub extern "C" fn ffi_sort_u8(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u8, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_u16(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u16, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_u32(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u32, n as usize)
    };
    to_sort.sort();
}
#[no_mangle]
pub extern "C" fn ffi_sort_u64(array_pointer: *const libc::c_void, n: libc::size_t) {
    assert!(!array_pointer.is_null());
    assert!(n != 0);
    let mut to_sort = unsafe {
        slice::from_raw_parts_mut(array_pointer as *mut u64, n as usize)
    };
    to_sort.sort();
}
