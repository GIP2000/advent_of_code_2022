use advent_of_code_2022::days::day10::{part_1, part_2};
use std::fs::read_to_string;
fn main() {
    let val = read_to_string("inputs/day10.txt").expect("Error reading file");
    println!("{}", part_1(&val));
    part_2(&val);
}
