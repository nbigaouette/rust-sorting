#!/usr/bin/env python3

import numpy as np
import copy as cp
import rust_sorting as rs

N = 5
max_val = 10.0

# dtype = np.int8
# dtype = np.int16
# dtype = np.int32
# dtype = np.int64

# dtype = np.uint8
# dtype = np.uint16
# dtype = np.uint32
# dtype = np.uint64

# dtype = np.float32
# dtype = np.float64

dtypes = [np.int8,  np.int16,  np.int32,  np.int64,
          np.uint8, np.uint16, np.uint32, np.uint64,
          np.float32, np.float64]


for dtype in dtypes:
    # print("dtype:", dtype)

    array = np.array(max_val*np.random.rand(N), dtype=dtype)

    orig_array = cp.deepcopy(array)

    # print("Python:      to sort =", repr(array))

    rs.quicksort(array)

    # print("Python:      sorted  =", repr(array))

    assert((array == np.sort(orig_array)).all())

print("Python done!")
