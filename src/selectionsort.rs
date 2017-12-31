//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of selection sort, in rust.
// https://en.wikipedia.org/wiki/Selection_sort

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
    for j in 0..NUM_VALUES - 1 {
        // We begin at j
        let mut min = j;

        // Find the minimum in the rest of the array, starting at j+1
        for i in j + 1..NUM_VALUES {
            if values[i] < values[min] {
                // Found a new minimum. Keep the index
                min = i;
            }
        }

        // If a minimum was found, swap it
        if min != j {
            let val1 = values[min];
            let val2 = values[j];
            values[min] = val2;
            values[j] = val1;
        }
    }

    // Print the sorted results
    println!("Sorted:");
    print_array(&values);
}
