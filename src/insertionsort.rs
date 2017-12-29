//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of insertion sort, in rust.
// https://en.wikipedia.org/wiki/Insertion_sort

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
    for i in 1..NUM_VALUES
    {
        let mut j = i;

        while j > 0 && values[j-1] > values[j]
        {
            let val1 = values[j];
            let val2 = values[j-1];
            values[j] = val2;
            values[j-1] = val1;

            j -= 1;
        }
    }

    // Print the sorted results
    println!("Sorted:");
    print_array(&values);
}