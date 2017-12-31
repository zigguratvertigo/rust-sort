//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

pub const NUM_VALUES: usize = 10;
pub const MAX_RANGE: u32 = 100;

// Copies an array into another, both of matching dimensions.
pub fn print_array(src: &[u32; NUM_VALUES]) {
    for i in 0..NUM_VALUES - 1 {
        print!("{}, ", src[i]);
    }
    print!("{}", src[NUM_VALUES - 1]);
}
