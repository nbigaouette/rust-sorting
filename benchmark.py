#!/usr/bin/env python3

import numpy as np
import copy as cp
import time

import rust_sorting as rs
import on_key

max_val = 10.0

dtype = np.int32

repeat = 10
Nb_power_of_two = 20
Ns = 2**np.arange(0, Nb_power_of_two)

timing = np.zeros((Nb_power_of_two, repeat), dtype=np.float64)

print("Ns:", Ns)

t1 = time.clock()
for Ni, N in enumerate(Ns):
    print("Ni:", Ni, "  N:", N)
    for r in range(0, repeat):
        array = np.array(max_val*np.random.rand(N), dtype=dtype)
        t0 = t1
        rs.quicksort(array)
        t1 = time.clock()
        timing[Ni, r] = t1 - t0

print("timing:", timing)

mean = np.mean(timing, axis=1)
std  = np.std(timing, axis=1)

print("mean:", mean)
print("std:", std)

assert(len(mean) == Nb_power_of_two)
assert(len(std)  == Nb_power_of_two)

fig = on_key.figure()
ax  = fig.add_subplot(1,1,1)
ax.errorbar(Ns, mean, yerr=std, label="Quicksort")
ax.grid(True)
ax.legend(loc='best')
ax.set_xlabel('N')
ax.set_ylabel('Duration [s]')
on_key.show()
