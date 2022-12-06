use std::collections::HashSet;

fn find_unique_b3nny_style(input: &str, amount: usize) -> Option<usize> {
    input
        .as_bytes()
        .windows(amount)
        .position(|arr| arr.iter().copied().collect::<HashSet<u8>>().len() == amount)
        .map(|n| n + amount)
}

fn find_unique_mine(input: &str, amount: usize) -> Option<usize> {
    for i in 0..(input.len() - amount) {
        let end = i + amount;
        if input[i..end].chars().collect::<HashSet<char>>().len() == amount {
            return Some(end);
        }
    }
    return None;
}

pub fn part_1(input: &str) -> usize {
    find_unique_b3nny_style(input, 4).unwrap()
}

pub fn part_2(input: &str) -> usize {
    find_unique_mine(input, 14).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
