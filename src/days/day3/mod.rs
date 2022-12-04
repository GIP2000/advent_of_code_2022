use crate::utill::chunk::ChunkIterator;
use std::collections::HashSet;

fn calculate(item: u8) -> u32 {
    return (if item as u8 > 'Z' as u8 {
        item - 'a' as u8 + 1
    } else {
        item - 'A' as u8 + 27
    }) as u32;
}
pub fn part_1(input: &str) -> u32 {
    return input
        .lines()
        .map(|bag| {
            let (first, second) = bag.split_at(bag.len() / 2);
            let first_map: HashSet<u8> = first.as_bytes().iter().cloned().collect();
            let second_map: HashSet<u8> = second.as_bytes().iter().cloned().collect();
            return second_map
                .iter()
                .filter_map(|&item| {
                    if !first_map.contains(&item) {
                        return None;
                    }
                    return Some(calculate(item));
                })
                .sum::<u32>();
        })
        .sum::<u32>();
}

pub fn part_2(input: &str) -> u32 {
    input
        .lines()
        .chunk()
        .filter_map(|[first, second, third]| {
            let first: HashSet<u8> = first.as_bytes().iter().cloned().collect();
            let second: HashSet<u8> = second.as_bytes().iter().cloned().collect();
            let third: HashSet<u8> = third.as_bytes().iter().cloned().collect();
            return Some(
                first
                    .iter()
                    .filter(|&v| second.contains(v) && third.contains(v))
                    .map(|&item| calculate(item))
                    .sum::<u32>(),
            );
        })
        .sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test() {
        assert_eq!(part_1(INPUT), 157);
        assert_eq!(part_2(INPUT), 70);
    }
}
