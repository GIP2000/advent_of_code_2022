use anyhow::{bail, Context, Result};
use std::{collections::HashSet, str::FromStr};

#[derive(Default, Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Vec2D(isize, isize);
impl std::ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn _draw_board(knots: &[Vec2D], min: Vec2D, max: Vec2D) {
    for y in (min.1..=max.1).rev() {
        for x in min.0..max.0 {
            let mut c = ".".to_string();
            if x == 0 && y == 0 {
                c = "s".to_string();
            }
            for (ki, k) in knots.iter().enumerate() {
                if *k == Vec2D(x, y) {
                    c = if ki == 0 {
                        "H".to_string()
                    } else {
                        format!("{}", ki)
                    };
                    break;
                }
            }
            print!("{}", c);
        }
        println!("");
    }
}

impl Vec2D {
    fn make_one(&self) -> Self {
        let mut n = *self;
        n.0 = self.0 / std::cmp::max(1, self.0.abs());
        n.1 = self.1 / std::cmp::max(1, self.1.abs());
        return n;
    }

    fn is_one_away(&self, rhs: Self) -> bool {
        let s = *self - rhs;
        return std::cmp::max(s.0.abs(), s.1.abs()) <= 1;
    }
}

impl FromStr for Vec2D {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Vec2D(0, 1)),
            "D" => Ok(Vec2D(0, -1)),
            "L" => Ok(Vec2D(-1, 0)),
            "R" => Ok(Vec2D(1, 0)),
            _ => {
                bail!("Invalid Direction")
            }
        }
    }
}

fn follow_knots<const N: usize>(input: &str) -> Result<usize> {
    let mut knots = [Vec2D(0, 0); N];
    let mut visited: HashSet<Vec2D> = HashSet::from([knots[0]]);

    for val in input.lines().map(|line| {
        line.split_once(" ")
            .context("Invalid Line")
            .map(|(dir, count)| -> Result<(Vec2D, isize)> {
                let dir = dir.parse::<Vec2D>()?;
                let count = count.parse::<isize>()?;
                return Ok((dir, count));
            })
    }) {
        let (dir, count) = val??;
        for _ in 0..count {
            knots[0] = knots[0] + dir;
            for i in 1..N {
                if knots[i - 1].is_one_away(knots[i]) {
                    continue;
                }
                knots[i] = knots[i] + (knots[i - 1] - knots[i]).make_one();
            }
            visited.insert(*knots.last().context("knots doesn't have a last value")?);
        }
    }

    return Ok(visited.len());
}

pub fn part_1(input: &str) -> usize {
    follow_knots::<2>(input).unwrap()
}

pub fn part_2(input: &str) -> usize {
    follow_knots::<10>(input).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT1: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const INPUT2: &'static str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT1), 13);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT2), 36);
    }
}
