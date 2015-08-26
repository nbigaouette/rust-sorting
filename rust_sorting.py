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
