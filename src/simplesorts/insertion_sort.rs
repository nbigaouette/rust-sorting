//! Insertion sort algorithm.
//!
//! The `insertion_sort` module contains the simple sorting algorithm "Insertion Sort".
//!
//! Source: https://en.wikipedia.org/wiki/Insertion_sort

use std::fmt::Debug;

pub fn sort<T: Debug + PartialOrd>(input: &Vec<T>) -> Vec<usize> {
    let n = input.len();

    let mut output_index: Vec<usize> = Vec::with_capacity(n);

    // Place the first element's index (0) as the first element of the output_index vector.
    output_index.push(0);

    println!("n = {:?}", n);
    println!("input:        {:?}", input);
    println!("output_index: {:?}", output_index);

    if n > 1 {
        // Loop over input vector, skipping the first element as it's already inserted as the first
        // element of output_index.
        for (i_start_at_0,elem) in input.iter().skip(1).enumerate() {
            let i = i_start_at_0 + 1;
            println!("  i={:?} Finding where to put element {:?} (i={:?}) in output_index: {:?}", i, elem, i, output_index);

            // Verify next element if it is smaller
            // If next element is larger than the last stored index, just place that element at
            // the end.
            if *elem >= input[*output_index.last().unwrap()] {
                // Next element "elem" is already larger than the previous stored index.
                // Just use its index.
                output_index.push(i);

                println!("    1. Pushing element '{:?}' at the end.", elem);
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
                println!("        3. output_index: {:?}", output_index);
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
