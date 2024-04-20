extern crate sortingLibrary;
use sortingLibrary::{merge_sort, quick_sort, insertion_sort, selection_sort};
fn main() {
    let mut numbers: Vec<i32> = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    println!("Original: {:?}", numbers);

    quick_sort(&mut numbers);
    println!("Quick Sorted: {:?}", numbers);

    let mut numbers2 = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    selection_sort(&mut numbers2);
    println!("Selection Sorted: {:?}", numbers2);

    let mut numbers3: Vec<i32> = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    insertion_sort(&mut numbers3);
    println!("Insertion Sorted: {:?}", numbers3);

    let mut numbers4 = vec![232, 111, 23, 9999, 1234, 1, 9, -7182, -14, 333];
    merge_sort(&mut numbers4);
    println!("Merge Sorted: {:?}", numbers4);
}