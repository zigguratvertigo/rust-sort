//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of insertion sort, in rust.
// https://en.wikipedia.org/wiki/Insertion_sort

// Externals
use common::{NUM_VALUES, MAX_RANGE, print_array};

use rand;
use rand::distributions::{IndependentSample, Range};

pub fn sort() {
    // Initialize the array values to random numbers
    let mut values: Vec<u32> = (0..NUM_VALUES)
        .map(|_| Range::new(0, MAX_RANGE).ind_sample(&mut rand::thread_rng()))
        .collect();

    // Print the original values
    println!("Original:");
    print_array(&values);
    println!("");

    // Sort
    for i in 1..NUM_VALUES {
        let mut j = i;

        while j > 0 && values[j - 1] > values[j] {
            let val1 = values[j];
            let val2 = values[j - 1];
            values[j] = val2;
            values[j - 1] = val1;

            j -= 1;
        }
    }

    // Print the sorted results
    println!("Sorted:");
    print_array(&values);
}
