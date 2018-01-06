//#![feature(rustc_private)]
extern crate rand;

mod bubblesort;
mod insertionsort;
mod mergesort;
mod selectionsort;
mod quicksort;

// Externals
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
    println!("{:?}\n", &values);

    println!("Bubble Sort:");
    let mut result_bubblesort: Vec<u32> = values.clone();
    bubblesort::sort(&mut result_bubblesort);
    println!("{:?}\n", &result_bubblesort);

    println!("Insertion Sort:");
    let mut result_insertionsort: Vec<u32> = values.clone();
    insertionsort::sort(&mut result_insertionsort);
    println!("{:?}\n", &result_insertionsort);

    println!("Merge Sort:");
    let mut result_mergesort: Vec<u32> = values.clone();
    mergesort::sort(&mut result_mergesort);
    println!("{:?}\n", &result_mergesort);

    println!("Selection Sort:");
    let mut result_selectionsort: Vec<u32> = values.clone();
    selectionsort::sort(&mut result_selectionsort);
    println!("{:?}\n", &result_selectionsort);

    println!("Quick Sort:");
    let mut result_quicksort: Vec<u32> = values.clone();
    quicksort::sort(&mut result_quicksort);
    println!("{:?}\n", &result_quicksort);
}

#[test]
fn sort_results_match() {
    // Initialize the array values to random numbers
    let values: Vec<u32> = (0..NUM_VALUES)
        .map(|_| Range::new(0, MAX_RANGE).ind_sample(&mut rand::thread_rng()))
        .collect();

    let mut result_bubblesort: Vec<u32> = values.clone();
    bubblesort::sort(&mut result_bubblesort);

    let mut result_insertionsort: Vec<u32> = values.clone();
    insertionsort::sort(&mut result_insertionsort);

    let mut result_mergesort: Vec<u32> = values.clone();
    mergesort::sort(&mut result_mergesort);

    let mut result_selectionsort: Vec<u32> = values.clone();
    selectionsort::sort(&mut result_selectionsort);

    let mut result_quicksort: Vec<u32> = values.clone();
    quicksort::sort(&mut result_quicksort);

    // test results by transitivity (A == B == C == D)
    assert!(result_bubblesort == result_insertionsort);
    assert!(result_insertionsort == result_mergesort);
    assert!(result_mergesort == result_selectionsort);
    assert!(result_selectionsort == result_quicksort);
}