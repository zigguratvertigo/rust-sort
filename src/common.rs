//
// MIT License
//
// Copyright (c) 2017 Colin Barré-Brisebois
//

// pub const NUM_VALUES: usize = 10;
// pub const MAX_RANGE: u32 = 100;

pub fn print_array(src: &[u32]) {
    for i in 0..src.len() {
        print!("{}, ", src[i]);
    }
}
