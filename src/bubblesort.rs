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
                let tmp = values[j - 1];
                values[j - 1] = values[j];
                values[j] = tmp;
            }
        }
    }
}
