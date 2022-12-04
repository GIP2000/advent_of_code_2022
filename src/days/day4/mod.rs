use std::collections::HashSet;
pub fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter(|elves| {
            let (elf1, elf2) = elves.split_once(",").unwrap();
            let elf1: Vec<u32> = elf1.split("-").flat_map(str::parse::<u32>).collect();
            let elf2: Vec<u32> = elf2.split("-").flat_map(str::parse::<u32>).collect();
            if elf1[0] <= elf2[0] && elf1[1] >= elf2[1] || elf2[0] <= elf1[0] && elf2[1] >= elf1[1]
            {
                return true;
            }
            false
        })
        .count()
}

pub fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|elves| {
            let (elf1, elf2) = elves.split_once(",").unwrap();
            let mut elf1: Vec<u32> = elf1.split("-").flat_map(str::parse::<u32>).collect();
            let mut elf2: Vec<u32> = elf2.split("-").flat_map(str::parse::<u32>).collect();
            if elf2[0] < elf1[0] {
                (elf1, elf2) = (elf2, elf1);
            }
            if elf2[0] <= elf1[1] {
                return true;
            }
            return false;
        })
        .count()
}
#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test() {
        assert_eq!(part_1(INPUT), 2);
        assert_eq!(part_2(INPUT), 4);
    }
}
