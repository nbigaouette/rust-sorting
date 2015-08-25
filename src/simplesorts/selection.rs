//! Selection sort algorithm.
//!
//! The `selection` module contains the simple sorting algorithm "Selection Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Selection_sort

use std::fmt::Debug;

/// Simple sort: selection sort.
///
/// # Details
///
/// Selection sort is one of the simplest sorting algorithm. The list to sort is split into two
/// sections; the first with the sorted element (starting empty) and the second with the remaining
/// element (starting full). At every iteration, the smallest element in the remaining list is
/// found and placed at the end of the sorted list.
///
/// This function takes an immutable reference vector of any type and returns a vector of `usize`
/// of the same length containing the indices of the initial vector sorted.
///
/// # Scaling
///
/// Selection sort in scale as O(N^2) since for all elements to sort, the algorithm will compare it
/// with every (remaining) elements.
///
/// Insertion sort is still useful as it is quite simple.
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
/// sorting::simplesorts::selection::sort(&mut data);
/// assert_eq!(vec![1, 2, 3, 4, 5], data);
/// ```
///
pub fn sort<T: Ord + Debug>(array: &mut Vec<T>) {
    println!("input array:  {:?}", array);

    if !array.is_empty() {
        // Let's loop over the input array, skipping last value (important as we want to get a
        // slice of the following values).
        let n = array.len();
        for i in 0..n-1 {
            println!("    i: {:?}", i);

            // Find index of the minimum value in the slice of values following the i^th element.
            let j = {
                let int_slice = &array[i+1..];
                println!("        array:        {:?}", array);
                println!("        int_slice:    {:?}", int_slice);
                // k now holds the index (of "array") of the minimum element of "int_slice", or the
                // value "i" of the current element of "array" (which should be the less than
                // all elements of "int_slice").
                let k = int_slice.iter().enumerate().fold(i, |j, pair| {
                    if array[i] > *pair.1 { i + 1 + pair.0 } else { j }
                });

                k
            };
            println!("            k: {:?}   array[i]={:?}   array[j]={:?}", j, array[i], array[j]);

            array.swap(i, j);
            println!("   new array: {:?}", array);
        }
    }

    println!("output array: {:?}", array);

    // panic!("Ok");
}
