//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of insertion sort, in rust.
// https://en.wikipedia.org/wiki/Insertion_sort

pub fn sort(values: &mut[u32]) {
    // Sort
    for i in 1..values.len() {
        let mut j = i;

        while j > 0 && values[j - 1] > values[j] {
            let val1 = values[j];
            let val2 = values[j - 1];
            values[j] = val2;
            values[j - 1] = val1;

            j -= 1;
        }
    }
}
