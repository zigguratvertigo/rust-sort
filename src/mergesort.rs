//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of merge sort, in rust.
// https://en.wikipedia.org/wiki/Merge_sort

// Externals
use common::{NUM_VALUES, MAX_RANGE, print_array};

use rand;
use rand::distributions::{IndependentSample, Range};
use std::cmp::min;

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
    bottom_up_merge_sort(&mut values);

    // Print the sorted results
    println!("Sorted:");
    print_array(&values);
}

// Bottom-up Merge Sort
//
// 1. Divide the unsorted list into n-sublists, each containing 1 element. A list of 1 element is considered sorted.
// 2. Repeatedly merge sublists to produce new sorted sublists until there is only 1 sublist remaining.
//    This will be the sorted list.
fn bottom_up_merge_sort(a: &mut [u32]) {
    // Initialize our work array
    let mut b = vec![0u32; a.len()];

    let len = a.len();

    let mut width = 1;
    while width < len {
        // Array A is full of runs of length width.
        let mut i = 0;
        while i < len {
            // Merge two runs: a[i:i+width-1] nd a[i+width:i+2*width-1] to b[]
            // or copy a[i:n-1] to b[] if(i+width >= n)
            bottom_up_merge(
                a,
                i,
                min(i + width, len),
                min(i + 2 * width, len),
                b.as_mut_slice(),
            );
            i = i + 2 * width;
        }

        // Now work array B is full of runs of length 2*width.
        // Copy array B to array A for next iteration.
        a.copy_from_slice(&b);

        // Now array A is full of runs of length 2*width.

        // Make successively longer sorted runs of length 2, 4, 8, 16... until whole array is sorted.
        width = 2 * width;
    }
}

fn bottom_up_merge(a: &mut [u32], i_left: usize, i_right: usize, i_end: usize, b: &mut [u32]) {
    let mut i = i_left;
    let mut j = i_right;

    for k in i_left..i_end {
        if i < i_right && (j >= i_end || a[i] <= a[j]) {
            b[k] = a[i];
            i = i + 1;
        } else {
            b[k] = a[j];
            j = j + 1;
        }
    }
}

