#!/usr/bin/env python3

import ctypes
import os
import glob

import numpy as np


# Load the Rust library when loading this module

target = "debug"
# target = "release"

libpath = os.path.join("target", target, "libsorting.*")
libfile = glob.glob(libpath)[0]
rustlib = ctypes.CDLL(libfile)

def sort(array):

    ptr = ctypes.c_void_p(array.ctypes.data)
    n   = len(array)

    if array.dtype == np.int8:
        rust_sort = rustlib.ffi_sort_i8
    elif array.dtype == np.int16:
        rust_sort = rustlib.ffi_sort_i16
    elif array.dtype == np.int32:
        rust_sort = rustlib.ffi_sort_i32
    elif array.dtype == np.int64:
        rust_sort = rustlib.ffi_sort_i64

    elif array.dtype == np.uint8:
        rust_sort = rustlib.ffi_sort_u8
    elif array.dtype == np.uint16:
        rust_sort = rustlib.ffi_sort_u16
    elif array.dtype == np.uint32:
        rust_sort = rustlib.ffi_sort_u32
    elif array.dtype == np.uint64:
        rust_sort = rustlib.ffi_sort_u64

    # Rust's vec.sort() can't be used on floating points!
    # elif array.dtype == np.float32:
    #     rust_sort = rustlib.ffi_sort_f32
    # elif array.dtype == np.float64:
    #     rust_sort = rustlib.ffi_sort_f64

    else:
        raise NotImplementedError

    rust_sort(ptr, n)

def quicksort(array):

    ptr = ctypes.c_void_p(array.ctypes.data)
    n   = len(array)

    if array.dtype == np.int8:
        rustlib.ffi_quicksort_i8(ptr, n)
    elif array.dtype == np.int16:
        rustlib.ffi_quicksort_i16(ptr, n)
    elif array.dtype == np.int32:
        rustlib.ffi_quicksort_i32(ptr, n)
    elif array.dtype == np.int64:
        rustlib.ffi_quicksort_i64(ptr, n)

    elif array.dtype == np.uint8:
        rustlib.ffi_quicksort_u8(ptr, n)
    elif array.dtype == np.uint16:
        rustlib.ffi_quicksort_u16(ptr, n)
    elif array.dtype == np.uint32:
        rustlib.ffi_quicksort_u32(ptr, n)
    elif array.dtype == np.uint64:
        rustlib.ffi_quicksort_u64(ptr, n)

    elif array.dtype == np.float32:
        rustlib.ffi_quicksort_f32(ptr, n)
    elif array.dtype == np.float64:
        rustlib.ffi_quicksort_f64(ptr, n)

    else:
         raise NotImplementedError
