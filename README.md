#Sorting Library
This Rust library provides sorting algorithms including quick sort, selection sort, insertion sort, and merge sort.

##Installation
##Create a new Cargo project by running the following command in your terminal:
cargo init sort

##Navigate into the newly created sort directory:
cd sort

##Open the Cargo.toml file in a text editor and add the following dependencies for your sorting library:

[dependencies]
sorting_library = { git = "https://github.com/raxaris/rust-sorting.git" }
Save the Cargo.toml file and close the text editor.

##Now you can import and use the sorting functions in your Rust code. Add the following code to your Rust file (e.g., main.rs):

use sorting_library::{quick_sort, selection_sort, insertion_sort, merge_sort};

##To build and run your Rust program, use the following command in the terminal:
cargo build
cargo run


License
MIT License

Copyright (c) 2024 Ansar Utenov
