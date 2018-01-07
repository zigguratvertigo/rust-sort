//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of shell sort, in rust.
// https://en.wikipedia.org/wiki/Shell_sort

pub fn sort(values: &mut[u32]) {
    // split the array into sections that decrease over iterations
    // todo: support custom gap distributions
    let mut gap = values.len() / 2;

    while gap > 0 {
        for i in gap..values.len() {
            let mut j = i;

            // modified insertion sort
            // todo: update insertionsort to allow for custom gaps
            while j >= gap && values[j - 1] > values[j] {
                let tmp = values[j - gap];
                values[j - gap] = values[j];
                values[j] = tmp;

                j -= gap;
            }
        }

        gap = gap / 2;
    }
}