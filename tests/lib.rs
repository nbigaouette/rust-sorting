extern crate sorting;

#[test]
fn it_works() {
    // let array: Vec<_> = vec![10, 11, 13, 14, 15, 12];
    let array: Vec<_> = vec![15, 14, 17, 12, 14, 19];
    // let array: Vec<_> = vec![5.2, 4.3, 7.4, 2.3, 4.1, 9.0];
    println!("it_works()...");
    sorting::simplesorts::insertion_sort(&array);
    println!("it_works() done!");
}
