# IntroToSubstrate_A3

## Description
This is the 3rd assignment of Introduction to Substrate, where we implement the BubbleSort algorithm with Rust. One (`bubble_sort_project`)is the most basic implementation sorting numbers, the other (`bubble_sort_generics`) can sort numbers and other types such as strings.

## Running the scripts
With Rust and Cargo installed in the local machine, the scripts can be cloned and run as the following:

For the basic implementation (sorting numbers only):
```
$ git clone https://github.com/YADengUU/IntroToSubstrate_A3
$ cd bubble_sort_project
$ cargo run
```
The output for sorting '3, 9, 5, 1, 11' shall be: [1, 3, 5, 9, 11]

For the other one (sorting generics), just change to the corresponding project directory `$ cd bubble_sort_generics` and do `$ cargo run`. In this simple example sorting the numbers '3, 9, 5, 1, 11' and words `"hamburger", "pasta", "soda", "chips"`, you shall see the following output
```
Sorted numbers: [1, 3, 5, 9, 11]
Sorted words: ["chips", "hamburger", "pasta", "soda"]
```

