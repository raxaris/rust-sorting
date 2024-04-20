extern crate sortingLibrary;
use sortingLibrary::{merge_sort, quick_sort, insertion_sort, selection_sort};
fn main() {
    let original: Vec<i32> = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    println!("Original: {:?}", original);

    let mut test = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    quick_sort(&mut test);
    println!("Quick Sort Result {:?}", test);

    let mut test2 = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    selection_sort(&mut test2);
    println!("Selection Sort Result {:?}", test2);

    let mut test3: Vec<i32> = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    insertion_sort(&mut test3);
    println!("Insertion Sort Result{:?}", test3);

    let mut test4 = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    merge_sort(&mut test4);
    println!("Merge Sort Result {:?}", test4);
}
