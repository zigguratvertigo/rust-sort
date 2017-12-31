//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of bubble sort, in rust.
// https://en.wikipedia.org/wiki/Bubble_sort

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
    for _ in 0..NUM_VALUES {
        for j in 1..NUM_VALUES {
            // if next value is bigger than previous, swap
            if values[j - 1] > values[j] {
                let val1 = values[j - 1];
                let val2 = values[j];

                values[j - 1] = val2;
                values[j] = val1;
            }
        }
    }

    // Print the sorted results
    println!("Sorted:");
    print_array(&values);
}
