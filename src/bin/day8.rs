use advent_of_code_2022::days::day8::{part_1, part_2};
use std::fs::read_to_string;
fn main() {
    let val = read_to_string("inputs/day8.txt").expect("Error reading file");
    println!("{}", part_1(&val));
    println!("{}", part_2(&val));
}
