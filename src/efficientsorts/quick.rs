//! Quicksort algorithm.
//!
//! The `efficient` module contains the efficient sorting algorithm "Quicksort".
//!
//! Source: https://en.wikipedia.org/wiki/Quicksort

use std::fmt::Debug;

/// Quicksort
///
/// # Details
///
///
///
/// # Scaling
///
///
///
/// # Optimizations
///
/// None
///
/// # Notes
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// # Examples
///
/// ```
/// let mut data: Vec<i32> = vec![4, 2, 3, 1, 5];
/// sorting::efficientsorts::quick::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
// pub fn sort<T: PartialOrd>(array: &mut Vec<T>) {
pub fn sort<T: PartialOrd + Debug>(array: &mut [T]) {
    let n = array.len();

    println!("");
    println!("  array: {:?}", array);

    if n <= 1 {
        return;
    } else if n == 2 {
        if array.first() <= array.last() {
            return;
        } else {
            array.swap(0, 1);
            // (array[0], array[1]) = (array[1], array[0]);
            return;
        }
    } else {
        // Pick a pivot: Pick the middle element by skipping half the length and keeping just one.
        // let it_pivot = array.iter().skip(n / 2).take(1);
        // Take the first element as the pivot.
        // let it_pivot = array.iter().take(1);
        //
        // for it in it_pivot {
        //     println!("  0. it: {:?}", it);
        // }

        // Loop over array, finding smaller elements than the pivot.
        // for it in array.iter().skip(1) {
        //     //println!("  1. it: {:?}", it);
        //     if it < it_pivot {
        //         println!("  1. it: {:?} smaller than pivot!", it);
        //     }
        // }
        // array.swap(0,1);

        // array.iter().enumerate().map(|(i,x)| {});

        // // Take the first element as pivot.
        // let a = array.iter_mut().skip(1).enumerate().fold(
        //     (0, array[0]), |acc, item| {
        //         if acc.1 > item.1 {
        //             // Element is smaller than pivot; swap them!
        //             array.swap(acc.0, item.0);
        //             // Return the accumulator as being the new index of pivot (which is now at
        //             // at the location of the fold()) with the pivot value which shouldn't change.
        //             (item.0, acc.1)
        //         } else {
        //             // Nothing to do as element is larger than pivot. Just return new accumulator.
        //             (acc.0, acc.1)
        //         }
        //     }
        // );

        let mut pivot = 0;
        for i in 1..n {
            println!("---------------");
            println!("  array[pivot={:?}]: {:?}    array[i={:?}]: {:?}", pivot, array[pivot], i, array[i]);
            if array[pivot] > array[i] {
                println!("    Swapping array[pivot={:?}]: {:?} with array[i={:?}]: {:?}", pivot, array[pivot], i, array[i]);
                array.swap(pivot, i);
                println!("  array: {:?}", array);
                if i == pivot+1 {
                    pivot = i;
                    println!("      pivot is now at {:?}", pivot);
                } else {
                    println!("      swapping i={:?} and pivot+1={:?}", i, pivot+1);
                    array.swap(i, pivot+1);
                    pivot = pivot+1;
                    println!("        pivot is now at {:?}", pivot);
                }
            }
            println!("  array: {:?}", array);
        }
        println!("array: {:?}", array);
        println!("array[pivot={:?}] = {:?}", pivot, array[pivot]);

        unimplemented!();
    }
}
