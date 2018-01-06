//
// MIT License
//
// Copy (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of quick sort, in rust.
// https://en.wikipedia.org/wiki/Quick_sort

pub fn sort(values: &mut [u32])
{
    let len: usize = values.len() - 1;

    // Sort
    quicksort(values, 0, len);
}

fn quicksort(values: &mut [u32], lo : usize, hi : usize) {
    let p = partition(values, lo, hi);

    if lo < p - 1 {
        quicksort(values, lo, p - 1);
    }

    if p < hi {
        quicksort(values, p, hi);
    }
}

fn partition(values: &mut [u32], lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi;

    // set the pivot to the middle
    let pivot = values[(lo + hi) / 2];

    while i <= j {
        while values[i] < pivot && i != values.len()-1 {
            i = i + 1;
        }

        while values[j] > pivot && j != 0 {
            j = j - 1;
        }

        if i <= j {
            let tmp = values[i];
            values[i] = values[j];
            values[j] = tmp;

            i = i + 1;

            if j > 0 {
                j = j - 1;
            }
        }
    }

    return i;
}