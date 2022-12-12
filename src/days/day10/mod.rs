pub fn part_1(input: &str) -> isize {
    let mut x: isize = 1;
    let mut sum: isize = 0;
    for (i, instruction) in input
        .lines()
        .flat_map(|instruction| {
            if instruction.starts_with("addx") {
                return vec!["noop", instruction];
            } else {
                return vec![instruction];
            }
        })
        .enumerate()
    {
        if i % 40 >= 19 && i % 40 - 19 == 0 {
            sum += x * (i as isize + 1);
        }
        if !instruction.starts_with("noop") {
            let (_, val) = instruction.split_once(" ").unwrap();
            let val = val.parse::<isize>().unwrap();
            x += val;
        }
    }
    sum
}

pub fn part_2(input: &str) {
    let mut x: isize = 1;
    for (i, instruction) in input
        .lines()
        .flat_map(|instruction| {
            if instruction.starts_with("addx") {
                return vec!["noop", instruction];
            } else {
                return vec![instruction];
            }
        })
        .enumerate()
    {
        if i % 40 == 0 {
            println!("");
        }
        let dif = x - (i % 40) as isize;
        print!("{}", if dif >= -1 && dif <= 1 { "#" } else { "." });
        if !instruction.starts_with("noop") {
            let (_, val) = instruction.split_once(" ").unwrap();
            let val = val.parse::<isize>().unwrap();
            x += val;
        }
    }
    println!("");
}

#[cfg(test)]
mod test {
    use super::*;
    //     const INPUT: &'static str = "noop
    // addx 3
    // addx -5";
    const INPUT: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13140);
    }
    #[test]
    fn test_part_2() {
        part_2(INPUT)
    }
}
