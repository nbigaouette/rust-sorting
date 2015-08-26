#!/usr/bin/env python3

import numpy as np
import rust_sorting as rs


array = np.zeros((5,), dtype=np.int8)


rs.quicksort(array)

print("Python done!")
