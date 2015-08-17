
// use std::fmt::Debug;

// pub fn insertion_sort<T: Debug>(array: &mut Vec<T>) {
pub fn insertion_sort(array: &mut Vec<f64>) {
    let n = array.len();

    if n > 1 {
        println!("  simplesorts::insertion_sort():  N = {:?}", n);
        for (i,elem) in array.iter().enumerate() {
            println!("    simplesorts::insertion_sort():  i={}   {:?}", i, elem);
        }
    }
}
