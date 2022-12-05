use std::collections::VecDeque;

fn parse_instruction(instruction: &str) -> Vec<usize> {
    instruction
        .split(" ")
        .flat_map(str::parse::<usize>)
        .map(|i| i - 1)
        .collect::<Vec<_>>()
}

fn build_original(original: &str) -> Vec<VecDeque<char>> {
    let len = original
        .lines()
        .rev()
        .next()
        .unwrap()
        .trim_end()
        .chars()
        .rev()
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap();

    let mut v: Vec<VecDeque<char>> = vec![VecDeque::new(); len as usize];
    for crates in original.lines() {
        let mut counter = 0;
        for (i, cr) in crates.split(" ").enumerate() {
            if cr.as_bytes().len() != 3 {
                if i % 4 == 0 {
                    counter += 1
                }
                continue;
            }
            v[counter % len as usize].push_back(cr.as_bytes()[1] as char);
            counter += 1;
        }
    }
    return v;
}

pub fn part_1(input: &str) -> String {
    let (original, rest) = input.split_once("\n\n").unwrap();
    let mut v = build_original(original);
    for instruction in rest.lines() {
        let instruction = parse_instruction(instruction);
        for _ in 0..=instruction[0] {
            let val = v[instruction[1]].pop_front().unwrap();
            v[instruction[2]].push_front(val);
        }
    }
    v.iter_mut()
        .map(|a| a.pop_front().unwrap())
        .fold("".to_string(), |acc, c| format!("{}{}", acc, c))
}

pub fn part_2(input: &str) -> String {
    let (original, rest) = input.split_once("\n\n").unwrap();
    let mut v = build_original(original);
    for instruction in rest.lines() {
        let instruction = parse_instruction(instruction);
        for val in (0..=instruction[0])
            .flat_map(|_| v[instruction[1]].pop_front())
            .collect::<Vec<_>>()
            .iter()
            .rev()
        {
            v[instruction[2]].push_front(*val);
        }
    }

    v.iter_mut()
        .map(|a| a.pop_front().unwrap())
        .fold("".to_string(), |acc, c| format!("{}{}", acc, c))
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), "CMZ");
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), "MCD");
    }
}
