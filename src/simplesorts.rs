
// use std::fmt::Debug;

// pub fn insertion_sort<T: Debug>(array: &mut Vec<T>) {
pub fn insertion_sort(array: &mut Vec<f64>) {
    let N = array.len();
    println!("  simplesorts::insertion_sort():  N = {:?}", N);
    for (i,elem) in array.iter().enumerate() {
        println!("    simplesorts::insertion_sort():  i={}   {:?}", i, elem);
    }
}
