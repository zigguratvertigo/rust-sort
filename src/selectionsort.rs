//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of selection sort, in rust.
// https://en.wikipedia.org/wiki/Selection_sort

pub fn sort(values: &mut [u32]) {
    // Sort
    for j in 0..values.len() - 1 {
        // We begin at j
        let mut min = j;

        // Find the minimum in the rest of the array, starting at j+1
        for i in j + 1..values.len() {
            if values[i] < values[min] {
                // Found a new minimum. Keep the index
                min = i;
            }
        }

        // If a minimum was found, swap it
        if min != j {
            let tmp = values[min];
            values[min] = values[j];
            values[j] = tmp;
        }
    }
}
