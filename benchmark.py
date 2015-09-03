#!/usr/bin/env python3

import numpy as np
import copy as cp
import time
import re
import os
import glob
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
Ns = np.unique(np.asarray(1.5**np.arange(0, Nn), dtype=int))
Nn = len(Ns)

fct_ptrs = [rs.sort,
            rs.quicksort, rs.mergesort, rs.heapsort,
            rs.insertionsort, rs.selectionsort, rs.bubblesort]


fct_names = [None]*len(fct_ptrs)
p = re.compile(r"<function (\w+) at")
for fi, f in enumerate(fct_ptrs):
    fct_names[fi] = p.match(str(f)).group(1)
del fi, f

timing = {}
tmp    = {}
for fi, f in enumerate(fct_ptrs):
    timing[fct_names[fi]] = None
    tmp[fct_names[fi]] = f
fct_ptrs = tmp
del tmp, fi, f


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
        print("Ni: %2d   N: %6d..." % (Ni, N), end="")
        for r in range(0, repeat):
            array = np.array(max_val*np.random.rand(N), dtype=dtype)
            t0 = t1
            fct_ptr(array)
            t1 = time.clock()
            data[Ni, r+1] = t1 - t0
        mean = np.mean(data[Ni, 1:])
        std  = np.std(data[Ni, 1:])

        print(" timing: %g +- %g (%.1f %%) s for %d repeats" % (mean, std, std/mean * 100, repeat))
    del Ni, N, r

    filename = get_filename(fct_name)
    header = "     N"
    fmt = "%8d"
    for r in range(0, repeat):
        header = "%s,   Run #%-2d [s]" % (header, r+1)
        fmt    = "%s, %%13.7e" % (fmt)
    del r
    np.savetxt(filename, data, header=header, fmt=fmt)

def load_benchmark(filename):
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
    del fct_name

    ax.grid(True)
    ax.legend(loc='best')
    ax.set_xlabel('N')
    ax.set_ylabel('Duration [s]')
    ax.set_xscale('log', basex=2)
    ax.set_yscale('log')
    xlims = ax.get_xlim()
    ax.set_xlim(xlims[0]/2, xlims[1]*2)
    ax.set_title('Scaling of different sorting algorithms implemented in Rust 1.2')
    on_key.show()

if not args.reload:
    for fct_name in fct_names:
        run_benchmark(fct_name)
    del fct_name

p = re.compile(os.path.join(benchmark_dir, r"(\w+)\.txt"))
files = glob.glob(os.path.join(benchmark_dir, "*"))
for f in files:
    fct_name = p.match(f).group(1)
    timing[fct_name] = load_benchmark(f)

plot_timing(timing)
