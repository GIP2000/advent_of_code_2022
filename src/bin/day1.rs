use advent_of_code_2022::days::day1::{get_3_maxes_heap, get_n_maxes, get_slow_n_maxes};
use std::fs::read_to_string;
use std::time::Instant;
fn main() {
    let val = read_to_string("inputs/day1.txt").expect("Error reading file");
    // println!("{}", get_n_maxes::<1>(&val));
    let before = Instant::now();
    println!("{}", get_n_maxes::<3>(&val));
    println!("fast = {:?}", before.elapsed());
    let before = Instant::now();
    println!("{}", get_slow_n_maxes(&val, 3));
    println!("slow = {:?}", before.elapsed());
    let before = Instant::now();
    println!("{}", get_3_maxes_heap(&val));
    println!("heap = {:?}", before.elapsed());
}
