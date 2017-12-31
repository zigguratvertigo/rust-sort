#![feature(rustc_private)]

extern crate rand;

mod common;
mod bubblesort;
mod insertionsort;
mod mergesort;
mod selectionsort;

fn main() {
    println!("bubblesort:");
    bubblesort::sort();
    println!("\n--------------------\n");

    println!("insertionsort:");
    insertionsort::sort();
    println!("\n--------------------\n");

    println!("mergesort:");
    mergesort::sort();
    println!("\n--------------------\n");

    println!("selectionsort:");
    selectionsort::sort();
    println!("\n--------------------\n");
}
