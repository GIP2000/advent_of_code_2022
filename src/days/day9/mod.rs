use std::collections::HashSet;

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
struct Vec2D(isize, isize);
impl std::ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<isize> for Vec2D {
    type Output = Vec2D;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::Sub for Vec2D {
    type Output = Vec2D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn signnum(n: isize) -> isize {
    if n > 0 {
        1
    } else {
        -1
    }
}

fn _draw_board(knots: &[Vec2D], min: Vec2D, max: Vec2D) {
    for y in (min.1..=max.1).rev() {
        for x in min.0..max.0 {
            let mut c = '.';
            if x == 0 && y == 0 {
                c = 's';
            }
            for (ki, k) in knots.iter().enumerate() {
                if *k == Vec2D(x, y) {
                    c = if ki != 0 {
                        char::from_digit(ki as u32, 10).unwrap()
                    } else {
                        'H'
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
        if self.0.abs() > 1 {
            n.0 = 1 * signnum(self.0);
        }
        if self.1.abs() > 1 {
            n.1 = 1 * signnum(self.1);
        }
        n
    }

    fn is_one_away(&self, rhs: Self) -> bool {
        *self + Self(1, 0) == rhs
            || *self + Self(0, 1) == rhs
            || *self + Self(-1, 0) == rhs
            || *self + Self(0, -1) == rhs
            || *self + Self(1, 1) == rhs
            || *self + Self(-1, -1) == rhs
            || *self + Self(-1, 1) == rhs
            || *self + Self(1, -1) == rhs
            || *self == rhs
    }
}

pub fn part_1(input: &str) -> usize {
    let mut h = Vec2D(0, 0);
    let mut t = Vec2D(0, 0);
    let mut visited: HashSet<Vec2D> = HashSet::new();
    visited.insert(t);

    for instruction in input.lines() {
        let (dir, count) = instruction.split_once(" ").unwrap();
        let count = count.parse::<isize>().unwrap();
        let dir = match dir {
            "U" => Vec2D(0, 1),
            "D" => Vec2D(0, -1),
            "L" => Vec2D(-1, 0),
            "R" => Vec2D(1, 0),
            _ => unreachable!(),
        };
        for _ in 0..count {
            h = h + dir;
            if h.is_one_away(t) {
                continue;
            }
            t = t + (h - t).make_one();
            visited.insert(t);
        }
    }
    visited.len()
}
pub fn part_2(input: &str) -> usize {
    let mut knots = [Vec2D(0, 0); 10];
    let mut visited: HashSet<Vec2D> = HashSet::new();
    visited.insert(knots[0]);

    for (dir, count) in input.lines().map(|line| line.split_once(" ").unwrap()) {
        let count = count.parse::<isize>().unwrap();
        let dir = match dir {
            "U" => Vec2D(0, 1),
            "D" => Vec2D(0, -1),
            "L" => Vec2D(-1, 0),
            "R" => Vec2D(1, 0),
            _ => unreachable!(),
        };

        for _ in 0..count {
            knots[0] = knots[0] + dir;
            for i in 1..knots.len() {
                if knots[i - 1].is_one_away(knots[i]) {
                    continue;
                }
                knots[i] = knots[i] + (knots[i - 1] - knots[i]).make_one();
            }
            visited.insert(*knots.last().unwrap());
        }
    }
    visited.len()
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
