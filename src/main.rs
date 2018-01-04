//#![feature(rustc_private)]
extern crate rand;

mod common;
mod bubblesort;
mod insertionsort;
mod mergesort;
mod selectionsort;

// Externals
use common::{print_array};

use rand::distributions::{IndependentSample, Range};

const NUM_VALUES: usize = 10;
const MAX_RANGE: u32 = 100;

fn main() {
    // Initialize the array values to random numbers
    let values: Vec<u32> = (0..NUM_VALUES)
        .map(|_| Range::new(0, MAX_RANGE).ind_sample(&mut rand::thread_rng()))
        .collect();

    // Print the original values
    println!("Original:");
    print_array(&values);
    println!("\n--------------------\n");

    println!("Bubble Sort:");
    let mut result_bubblesort: Vec<u32> = values.clone();
    bubblesort::sort(&mut result_bubblesort);
    print_array(&result_bubblesort);
    println!("\n--------------------\n");

    println!("Insertion Sort:");
    let mut result_insertionsort: Vec<u32> = values.clone();
    insertionsort::sort(&mut result_insertionsort);
    print_array(&result_insertionsort);
    println!("\n--------------------\n");

    println!("Merge Sort:");
    let mut result_mergesort: Vec<u32> = values.clone();
    mergesort::sort(&mut result_mergesort);
    print_array(&result_mergesort);
    println!("\n--------------------\n");

    println!("Selection Sort:");
    let mut result_selectionsort: Vec<u32> = values.clone();
    mergesort::sort(&mut result_selectionsort);
    print_array(&result_selectionsort);
    println!("\n--------------------\n");
}
