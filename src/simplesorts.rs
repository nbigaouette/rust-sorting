
use std::fmt::Debug;

pub fn insertion_sort<T: Clone + Debug>(input: &[T]) -> &[T] {
    let n = input.len();

    let output = input.clone();

    println!("  simplesorts::insertion_sort():  n = {:?}", n);
    println!("  simplesorts::insertion_sort():  input:  {:?}", input);
    println!("  simplesorts::insertion_sort():  output: {:?}", output);

    for (i,elem) in input.iter().enumerate() {
        println!("    simplesorts::insertion_sort():  i={}   {:?}", i, elem);
    }

    return output;
}
