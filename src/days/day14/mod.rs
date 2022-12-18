use std::{collections::BinaryHeap, str::FromStr};

use anyhow::{Context, Result};

#[derive(Debug)]
struct Cave {
    rocks: Vec<Vec<usize>>,
    source: (usize, usize),
    largest: usize,
}

impl Cave {
    pub fn draw(&self, start: (usize, usize), end: (usize, usize)) -> String {
        let mut s = "".to_string();

        for x in start.1..=end.1 {
            let mut row = "".to_string();
            for y in start.0..=end.0 {
                if (x, y) == self.source {
                    row = format!("{}+", row);
                } else if let Some(_) = self.rocks[y].iter().find(|&&xi| xi == x) {
                    row = format!("{}#", row);
                } else {
                    row = format!("{}.", row);
                }
            }
            s = format!("{}\n{}", s, row);
        }
        return s;
    }
    pub fn simulate_part_2(&mut self) -> usize {
        let mut counter = 0;
        while let Some(l) = self.drop(self.source, true) {
            // can be optomized, im not going to this can be an insertion sort which will be ~O(N)
            // cause of mostly sorted
            self.rocks[l.0].push(l.1);
            self.rocks[l.0].sort();
            counter += 1;
            if l == self.source {
                return counter;
            }
        }
        unreachable!("Error in Cave");
    }
    pub fn simulate_part_1(&mut self) -> usize {
        let mut counter = 0;
        while let Some(l) = self.drop(self.source, false) {
            // can be optomized, im not going to this can be an insertion sort which will be ~O(N)
            // cause of mostly sorted
            self.rocks[l.0].push(l.1);
            self.rocks[l.0].sort();
            counter += 1;
        }
        return counter;
    }

    fn drop(&mut self, from: (usize, usize), has_floor: bool) -> Option<(usize, usize)> {
        let pnt = *self.rocks[from.0].iter().find(|pot| {
            if **pot > from.1 {
                return true;
            }
            return false;
        })?;
        if pnt == self.largest && !has_floor {
            return None;
        }

        // check if it can go left
        // let l_list = self.rocks.get(from.0 - 1)?; // might underflow?

        let l_list = match (has_floor, self.rocks.get(from.0 - 1)) {
            (_, Some(v)) => Some(v),
            (true, None) => {
                self.rocks.insert(from.0 - 1, vec![self.largest]);
                Some(&self.rocks[from.0 - 1])
            }
            (false, None) => None,
        }?;
        let mut no_left = false;
        for y in l_list.iter() {
            if *y == pnt {
                no_left = true;
                break;
            }
        }
        if !no_left {
            return self.drop((from.0 - 1, pnt), has_floor);
        }
        let r_list = match (has_floor, self.rocks.get(from.0 + 1)) {
            (_, Some(v)) => Some(v),
            (true, None) => {
                self.rocks.insert(from.0 + 1, vec![self.largest]);
                Some(&self.rocks[from.0 + 1])
            }
            (false, None) => None,
        }?;
        let mut no_right = false;
        for y in r_list.iter() {
            if *y == pnt {
                no_right = true;
                break;
            }
        }
        if !no_right {
            return self.drop((from.0 + 1, pnt), has_floor);
        }

        return Some((from.0, pnt - 1));
    }
}

impl FromStr for Cave {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x_vec = vec![BinaryHeap::new(); 1000];
        let mut largest = 0;
        for lines in s.lines() {
            for window in lines
                .split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(",").context("Invalid Point")?;
                    Ok((x.parse::<usize>()?, y.parse::<usize>()?))
                })
                .collect::<Result<Vec<_>>>()?
                .as_slice()
                .windows(2)
            {
                let start = window[0];
                let end = window[1];
                if start.0 == end.0 {
                    if x_vec.len() <= start.0 {
                        x_vec.resize(start.0 + 1, BinaryHeap::new());
                    }
                    let (s, e) = if start.1 > end.1 {
                        (end.1, start.1)
                    } else {
                        (start.1, end.1)
                    };
                    for v in (s..=e).rev() {
                        x_vec[start.0].push(v);
                        largest = std::cmp::max(v, largest);
                    }
                    continue;
                }
                let (s, e) = if start.0 > end.0 {
                    (end.0, start.0)
                } else {
                    (start.0, end.0)
                };
                for v in s..=e {
                    if x_vec.len() <= v {
                        x_vec.resize(v + 1, BinaryHeap::new());
                    }
                    x_vec[v].push(start.1);
                    largest = std::cmp::max(start.1, largest);
                }
            }
        }
        largest += 2;
        let mut x_vec = x_vec
            .into_iter()
            .map(|bh| bh.into_sorted_vec())
            .collect::<Vec<_>>();

        for x in x_vec.iter_mut() {
            x.push(largest);
        }

        return Ok(Self {
            source: (500, 0),
            rocks: x_vec,
            largest,
        });
    }
}

pub fn part_1(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    return cave.simulate_part_1();
}
pub fn part_2(input: &str) -> usize {
    let mut cave: Cave = input.parse().unwrap();
    return cave.simulate_part_2();
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 24);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 93);
    }
}
