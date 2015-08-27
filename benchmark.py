#!/usr/bin/env python3

import numpy as np
import copy as cp
import time
import re
import os
import argparse

import rust_sorting as rs
import on_key


parser = argparse.ArgumentParser(description='Benchmark sorting algorithms.')
parser.add_argument('-r', '--reload', action='store_true',
                    help="Don't run the benchmarks; just reload data and plot.")

args = parser.parse_args()


max_val = 10.0

dtype = np.int32

benchmark_dir = "benchmark"

repeat = 10
Nn = 30
Ns = np.asarray(1.5**np.arange(0, Nn), dtype=int)

fct_ptrs = [rs.sort, rs.quicksort, rs.insertionsort, rs.selectionsort]


fct_names = [None]*len(fct_ptrs)
p = re.compile(r"<function (\w+) at")
for fi, f in enumerate(fct_ptrs):
    fct_names[fi] = p.match(str(f)).group(1)

timing = {}
tmp    = {}
for fi, f in enumerate(fct_ptrs):
    timing[fct_names[fi]] = None
    tmp[fct_names[fi]] = f
fct_ptrs = tmp


def get_filename(fct_name):
    filename = os.path.join(benchmark_dir, "%s.txt" % fct_name)
    return filename

def run_benchmark(fct_name):
    fct_ptr = fct_ptrs[fct_name]
    print(fct_name)

    data = np.zeros((Nn, 1+repeat), dtype=np.float64)

    data[:, 0] = Ns

    t1 = time.clock()
    for Ni, N in enumerate(Ns):
        print("Ni:", Ni, "  N:", N)
        for r in range(0, repeat):
            array = np.array(max_val*np.random.rand(N), dtype=dtype)
            t0 = t1
            f(array)
            t1 = time.clock()
            data[Ni, r+1] = t1 - t0

    filename = get_filename(fct_name)
    header = "     N"
    fmt = "%8d"
    for r in range(0, repeat):
        header = "%s,   Run #%-2d [s]" % (header, r+1)
        fmt    = "%s, %%13.7e" % (fmt)
    np.savetxt(filename, data, header=header, fmt=fmt)

def load_benchmark(fct_name):
    filename = get_filename(fct_name)
    data = np.loadtxt(filename, delimiter=',')
    return data

def plot_timing(data):
    fig = on_key.figure()
    ax  = fig.add_subplot(1,1,1)

    for fct_name in data:

        N = data[fct_name][:,0]
        T = data[fct_name][:,1:]

        mean = np.mean(T, axis=1)
        std  = np.std(T,  axis=1)

        assert(len(N) == len(mean))
        assert(len(N) == len(std))

        ax.errorbar(N, mean, yerr=std, label=fct_name)

    ax.grid(True)
    ax.legend(loc='best')
    ax.set_xlabel('N')
    ax.set_ylabel('Duration [s]')
    on_key.show()

if not args.reload:
    for fct_name in fct_names:
        run_benchmark(fct_name)

for fct_name in fct_names:
    timing[fct_name] = load_benchmark(fct_name)

plot_timing(timing)
