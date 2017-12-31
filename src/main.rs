extern crate rand;

mod common;
mod bubblesort;
mod insertionsort;
mod mergesort;
mod selectionsort;

fn main() {
    println!("bubblesort:");
    bubblesort::main();
    println!("\n--------------------\n");

    println!("insertionsort:");
    insertionsort::main();
    println!("\n--------------------\n");

    println!("mergesort:");
    mergesort::main();
    println!("\n--------------------\n");

    println!("selectionsort:");
    selectionsort::main();
    println!("\n--------------------\n");
}
