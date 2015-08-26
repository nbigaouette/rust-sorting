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
    rustlib.ffi_quicksort_i8(ctypes.c_void_p(array.ctypes.data), len(array))
