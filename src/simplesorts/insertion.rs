//! Insertion sort algorithm.
//!
//! The `insertion_sort` module contains the simple sorting algorithm "Insertion Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Insertion_sort

use std::fmt::Debug;


/// Simple sort: insertion sort.
///
/// # Details
///
/// Insertion sort is one of the simplest sorting algorithm. Every elements of the list to sort
/// are picked one at a time and inserted into a the sorted list at the right location.
///
/// This function takes an immutable reference vector of any type and returns a vector of `usize`
/// of the same length containing the indices of the initial vector sorted.
///
/// # Scaling
///
/// In the best case scenario, the list is already sorted and the work is simply to append the
/// element at the end of the list, hence a O(N) scaling.
///
/// In the worst case scenario, the list is in reverse order. Hence, every element is compared to
/// all previous elements already sorted until this new one gets inserted at the beginning of the
/// sorted list, thus making this case O(N^2).
///
/// Insertion sort is still useful as its simplicity (and thus small overhead) makes it ideal for
/// small vectors.
///
/// # Notes
///
/// The implementation is 'unstable' as it does not preserve the relative order of items with
/// equal values. It could be made stable though if required.
///
/// The type T of the vector elements to sort _must_ implement the `PartialOrd` trait so the
/// compiler knows how to compare the elements and sort them.
///
/// # Examples
///
/// ```
/// let data: Vec<i32> = vec![4, 2, 3, 1];
/// assert_eq!(vec![3, 1, 2, 0], sorting::simplesorts::insertion::sort(&data));
/// ```
///
pub fn sort<T: Debug + PartialOrd>(input: &Vec<T>) -> Vec<usize> {
    let n = input.len();

    // Declare the vector of indices to return. Reserve memory for "n" element so it
    // is the same size as the vector to sort.
    let mut output_index: Vec<usize> = Vec::with_capacity(n);

    if !input.is_empty() {

        // Place the first element's index (0) as the first element of the output_index vector.
        output_index.push(0);

        // Loop over input vector, skipping the first element as it's already inserted as the first
        // element of output_index.
        for (i_start_at_0,elem) in input.iter().skip(1).enumerate() {
            // enumerate() returns the current index of iteration, not the index of the
            // vector iterated upon. This means it will start at 0, even though we wanted to skip
            // the first element using skip(1). Let's get the value we wanted: 'i', the loop index
            // of the input array.
            let i = i_start_at_0 + 1;

            // Compare the element of the loop 'elem' with the last element stored in the index
            // vector 'output_index'.
            if *elem >= input[*output_index.last().unwrap()] {
                // If the element of the loop is larger or equal to the last sorted element, simply
                // append its index to the sorted index list.
                output_index.push(i);
            } else {
                println!("    2. elem ?< input[output_index[0]]");
                println!("           {:?} ?< {:?}", elem, input[output_index[0]]);
                // The next element of the input vector is smaller than the last stored on.
                // Let's find where it should go by looping back on the stored indices.
                // NOTE: we must use a loop on indices as to prevent problems with the borrow-checker.
                if *elem < input[output_index[0]] {
                    println!("        2a. Yes: --> putting i in front");
                    output_index.insert(0, i);
                } else {
                    println!("        2b. No: --> Looping back");
                    for j in (0..i).rev() {
                        println!("            j={:?}   elem={:?}   input[output_index[j]]={:?}", j, elem, input[output_index[j]]);
                        if *elem > input[output_index[j]] {
                            output_index.insert(j+1, i);
                            break;
                        }
                    }
                }
            }

            print!("           output: [");
            for j in output_index.iter() {
                    print!("{:?}, ", input[*j]);
            }
            println!("]");
        }
    }

    println!("");
    println!("END: input:        {:?}", input);
    println!("END: output_index: {:?}", output_index);
    print!("END: output:       [");
    for j in output_index.iter() {
            print!("{:?}, ", input[*j]);
    }
    println!("]");


    assert_eq!(output_index.len(), n);

    return output_index;
}
