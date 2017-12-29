//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of selection sort, in rust.
// https://en.wikipedia.org/wiki/Selection_sort

#![feature(rustc_private)]

// Externals
mod common;
use common::NUM_VALUES;
use common::MAX_RANGE;
use common::print_array;

extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn main()
{
    // Create the array that will contain all of our values
    let mut values: [u32; NUM_VALUES] = [0; NUM_VALUES];

    // Initialize a random number generator that goes between 0..MAX_RANGE
    let range = Range::new(0, MAX_RANGE);
    let mut rng = rand::thread_rng();

    // Initialize the array values to random numbers
    for n in 0..NUM_VALUES
    {
        values[n] = range.ind_sample(&mut rng);
    }

    // Print the original values
    println!("Original:");
    print_array(&values);
    println!("");

    // Sort
    for j in 0..NUM_VALUES-1
    {
        // We begin at j
        let mut min = j;

        // Find the minimum in the rest of the array, starting at j+1
        for i in j+1..NUM_VALUES
        {
            if values[i] < values[min]
            {
                // Found a new minimum. Keep the index
                min = i;
            }
        }

        // If a minimum was found, swap it
        if min != j
        {
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