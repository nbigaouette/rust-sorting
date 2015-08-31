#!/usr/bin/env python3

import numpy as np
import copy as cp
import rust_sorting as rs

N = 5
max_val = 10.0

dtypes = [np.int8,  np.int16,  np.int32,  np.int64,
          np.uint8, np.uint16, np.uint32, np.uint64,
          np.float32, np.float64]


for dtype in dtypes:
    # print("dtype:", dtype)

    array = np.array(max_val*(np.random.rand(N) - 0.5), dtype=dtype)

    orig_array = cp.deepcopy(array)

    # print("Python:      to sort =", repr(array))

    # rs.sort(array)
    # rs.insertionsort(array)
    # rs.selectionsort(array)
    # rs.quicksort(array)
    # rs.mergesort(array)
    rs.heapsort(array)

    # print("Python:      sorted  =", repr(array))

    assert((array == np.sort(orig_array)).all())

print("Python done and sorted!")
