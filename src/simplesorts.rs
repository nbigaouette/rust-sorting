
use std::fmt::Debug;

pub fn insertion_sort<T: Debug>(array: &[T]) /*-> &[T]*/ {
    for elem in array {
        println!("    simplesorts::insertion_sort():  {:?}", elem);
    }
}
