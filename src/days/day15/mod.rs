use anyhow::{Context, Result};
use std::str::FromStr;

#[derive(Debug)]
struct Sensor {
    pos: (isize, isize),
    closest_beacon: (isize, isize),
}

impl Sensor {
    pub fn distance(&self) -> isize {
        return (self.pos.0 - self.closest_beacon.0).abs()
            + (self.pos.1 - self.closest_beacon.1).abs();
    }
}
impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let v = s
            .split(":")
            .map(|v| {
                let (x, y) = v.split_once(",").context("Invalid Str")?;
                let (_, x) = x.split_once("x=").context("Can't find x")?;
                let (_, y) = y.split_once("y=").context("Can't find x")?;
                let x = x.parse()?;
                let y = y.parse()?;
                return Ok((x, y));
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            pos: v[0],
            closest_beacon: v[1],
        })
    }
}

fn find_slice(input: &str, y_slice: isize) -> usize {
    let mut left_bound = isize::MAX;
    let mut right_bound = isize::MIN;
    let sensors: Vec<Sensor> = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()
        .unwrap();
    for s in sensors.iter() {
        let center_dist = s.distance() - (s.pos.1 - y_slice).abs();
        let cur_left = s.pos.0 - center_dist;
        let cur_right = s.pos.0 + center_dist;
        left_bound = std::cmp::min(cur_left, left_bound);
        right_bound = std::cmp::max(cur_right, right_bound);
    }
    return (right_bound - left_bound) as usize;
}

fn find_free(input: &str, max_c: isize) -> usize {
    let mut bounds: Vec<Vec<(isize, isize)>> = vec![vec![]; max_c as usize + 1];
    let sensors: Vec<Sensor> = input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>>>()
        .unwrap();

    for y_slice in 0..(max_c as usize) {
        println!("checking slice {y_slice}/{max_c}");
        for s in sensors.iter() {
            let true_dist = s.distance();
            if (s.pos.1 - y_slice as isize).abs() > true_dist {
                // println!("skipping: {s:?}");
                continue;
            }
            let center_dist = (true_dist - (s.pos.1 - y_slice as isize).abs()).abs();
            let cur_bounds = (s.pos.0 - center_dist, s.pos.0 + center_dist);

            // if y_slice == 11 {
            // println!(
            //     "pos: {:?}, true_distance: {:?}, center_dist: {:?}, cur_bouunds: {:?}",
            //     s.pos, true_dist, center_dist, cur_bounds
            // );
            // }

            bounds[y_slice].push(cur_bounds);
            // try to combine things in bounds
            'w_loop: while bounds[y_slice].len() > 1 {
                for i in 0..bounds[y_slice].len() {
                    for j in 0..bounds[y_slice].len() {
                        if i == j {
                            continue;
                        }
                        // println!("i: {:?}, j: {:?}", bounds[y_slice][i], bounds[y_slice][j]);
                        // check if i is inside j
                        if bounds[y_slice][i].0 >= bounds[y_slice][j].0
                            && bounds[y_slice][i].1 <= bounds[y_slice][j].1
                        {
                            // println!(
                            //     "{:?} is inside {:?}",
                            //     bounds[y_slice][i], bounds[y_slice][j]
                            // );
                            bounds[y_slice].remove(i);
                            // println!("bounds[{:?}] = {:?}", y_slice, bounds[y_slice]);
                            continue 'w_loop;
                        }
                        // check if j is inside i
                        if bounds[y_slice][j].0 >= bounds[y_slice][i].0
                            && bounds[y_slice][j].1 <= bounds[y_slice][i].1
                        {
                            // println!(
                            //     "{:?} is inside {:?}",
                            //     bounds[y_slice][j], bounds[y_slice][i]
                            // );
                            bounds[y_slice].remove(j);
                            // println!("bounds[{:?}] = {:?}", y_slice, bounds[y_slice]);
                            continue 'w_loop;
                        }
                        // check if i is left extend j
                        if bounds[y_slice][i].0 <= bounds[y_slice][j].0
                            && bounds[y_slice][i].1 >= bounds[y_slice][j].0
                        // && bounds[y_slice][i].1 <= bounds[y_slice][j].1
                        {
                            // println!(
                            //     "{:?} is left extend of {:?}",
                            //     bounds[y_slice][i], bounds[y_slice][j]
                            // );
                            bounds[y_slice][j].0 = bounds[y_slice][i].0;
                            bounds[y_slice].remove(i);
                            // println!("bounds[{:?}] = {:?}", y_slice, bounds[y_slice]);
                            continue 'w_loop;
                        }
                        // check if i is right extend j
                        if bounds[y_slice][i].1 >= bounds[y_slice][j].1
                            && bounds[y_slice][i].0 <= bounds[y_slice][j].1
                        {
                            // println!(
                            //     "{:?} is right extend of {:?}",
                            //     bounds[y_slice][i], bounds[y_slice][j]
                            // );
                            bounds[y_slice][j].1 = bounds[y_slice][i].1;
                            bounds[y_slice].remove(i);
                            // println!("bounds[{:?}] = {:?}", y_slice, bounds[y_slice]);
                            continue 'w_loop;
                        }
                    }
                }
                break;
            }
        }
        if bounds[y_slice].len() > 1 {
            println!("Found: bounds[{:?}] = {:?}", y_slice, bounds[y_slice]);
            return (y_slice as isize
                + (std::cmp::min(bounds[y_slice][0].1, bounds[y_slice][1].1) + 1) * 4000000)
                as usize;
        }
    }
    return 0;
}

pub fn part_1(input: &str) -> usize {
    return find_slice(input, 2000000);
}
pub fn part_2(input: &str) -> usize {
    return find_free(input, 4000000);
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part_1() {
        assert_eq!(find_slice(INPUT, 10), 26);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(find_free(INPUT, 20), 56000011);
    }
}
