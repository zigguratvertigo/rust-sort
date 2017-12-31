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

pub fn main() {
    // Create the array that will contain all of our values
    let mut values: [u32; NUM_VALUES] = [0; NUM_VALUES];

    // Initialize a random number generator that goes between 0..MAX_RANGE
    let range = Range::new(0, MAX_RANGE);
    let mut rng = rand::thread_rng();

    // Initialize the array values to random numbers
    for n in 0..NUM_VALUES {
        values[n] = range.ind_sample(&mut rng);
    }

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
