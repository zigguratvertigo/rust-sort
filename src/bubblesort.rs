//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of bubble sort, in rust.
// https://en.wikipedia.org/wiki/Bubble_sort

pub fn sort(values: &mut[u32]) {
    // Sort
    for _ in 0..values.len() {
        for j in 1..values.len() {
            // if next value is bigger than previous, swap
            if values[j - 1] > values[j] {
                let val1 = values[j - 1];
                let val2 = values[j];

                values[j - 1] = val2;
                values[j] = val1;
            }
        }
    }
}
