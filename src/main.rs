#![allow(unused)]
#![feature(int_to_from_bytes)]

mod datastruct;
use std::time::Instant;
use datastruct::*;

fn main() {
    let start = Instant::now();
    let bin = read_datastruct_from_file("DataStructFile.bin");
    let elapsed = start.elapsed();
    // debug format:
    println!("{:?}", elapsed);
}
