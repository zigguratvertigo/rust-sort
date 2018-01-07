//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// Basic implementation of heap sort, in rust.
// https://en.wikipedia.org/wiki/Heap_sort

pub fn sort(values: &mut [u32]) {
    let n = values.len();

    let mut i = n / 2;
    loop {
        heapify(values, n, i);

        if i >= 1 {
            i = i - 1;
        } else {
            break;
        }
    }

    i = n-1;
    loop {
        let tmp = values[0];
        values[0] = values[i];
        values[i] = tmp;

        heapify(values, i, 0);

        if i >= 1 {
            i = i - 1;
        } else {
            break;
        }
    }
}

fn heapify(values: &mut [u32], n: usize, i: usize)
{
    let mut largest = i;
    let l = 2 * i + 1;
    let r = 2 * i + 2;

    if l < n && values[l] > values[largest]
    {
        largest = l;
    }

    if r < n && values[r] > values[largest]
    {
        largest = r;
    }

    if largest != i
    {
        let tmp = values[i];
        values[i] = values[largest];
        values[largest] = tmp;

        heapify(values, n, largest);
    }
}