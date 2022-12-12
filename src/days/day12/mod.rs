use anyhow::{Context, Result};
use std::collections::BinaryHeap;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{ops::Index, str::FromStr};

type Cords = (usize, usize);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Node {
    loc: Cords,
    f: u32,
}

impl Add for Node {
    type Output = Node;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_root((self.loc.0 + rhs.loc.0, self.loc.1 + rhs.loc.1))
    }
}

impl AddAssign for Node {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Node {
    type Output = Node;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_root((self.loc.0 - rhs.loc.0, self.loc.1 - rhs.loc.1))
    }
}
impl SubAssign for Node {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f.cmp(&self.f).then_with(|| self.loc.cmp(&other.loc))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    pub fn new(loc: Cords, f: u32) -> Self {
        Self { loc, f }
    }

    pub fn new_root(loc: Cords) -> Self {
        Self { loc, f: u32::MAX }
    }
}

#[derive(Debug)]
struct Board {
    map: Vec<Vec<(u8, Option<Cords>)>>,
    start: Cords,
    end: Cords,
}

impl Board {
    fn get_succesors(&self, q_node: Node) -> Vec<Node> {
        let cv = self[q_node];
        let mut successors = vec![];
        let mut check = |node: Node, add: bool| {
            let v_node = if add {
                Some(q_node + node)
            } else {
                if q_node.loc.0 < node.loc.0 || q_node.loc.1 < node.loc.1 {
                    None
                } else {
                    Some(q_node - node)
                }
            };
            let v = v_node.map(|v_node| self.get(v_node));
            match v {
                Some(Some(&v)) => {
                    if v <= cv + 1 {
                        let mut new_node = v_node.unwrap();
                        new_node.f = ((q_node.loc.0 as i32 - new_node.loc.0 as i32).abs()
                            + (q_node.loc.1 as i32 - new_node.loc.1 as i32).abs()
                            + (self[q_node] as i32 - self[new_node] as i32).abs())
                            // + (q_node.loc.0 as i32 - self.end.0 as i32).abs()
                            // + (q_node.loc.1 as i32 - self.end.1 as i32).abs()
                            // + (self[q_node] as i32 - self[self.end] as i32).abs())
                            as u32;
                        successors.push(new_node);
                    }
                }
                _ => {}
            };
        };
        check(Node::new_root((1, 0)), true);
        check(Node::new_root((1, 0)), false);
        check(Node::new_root((0, 1)), true);
        check(Node::new_root((0, 1)), false);
        return successors;
    }

    fn get(&self, node: Node) -> Option<&u8> {
        Some(&self.map.get(node.loc.0).map(|r| r.get(node.loc.1))??.0)
    }

    fn set_parent(&mut self, loc: Cords, parent: Cords) {
        self.map[loc.0][loc.1].1 = Some(parent);
    }
    fn get_parent(&self, loc: Cords) -> Option<Cords> {
        self.map[loc.0][loc.1].1
    }
}

impl Index<Cords> for Board {
    type Output = u8;

    fn index(&self, index: Cords) -> &Self::Output {
        &self.map[index.0][index.1].0
    }
}

impl Index<Node> for Board {
    type Output = u8;

    fn index(&self, index: Node) -> &Self::Output {
        &self[index.loc]
    }
}

impl FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let map = s
            .trim()
            .lines()
            .enumerate()
            .map(|(y, s)| {
                s.trim()
                    .bytes()
                    .enumerate()
                    .map(|(x, b)| {
                        if b == 'S' as u8 {
                            start = Some((y, x));
                            return (0u8, None);
                        } else if b == 'E' as u8 {
                            end = Some((y, x));
                            return (('z' as u8) - ('a' as u8), None);
                        } else {
                            return (b - 'a' as u8, None);
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        return Ok(Self {
            map,
            start: start.context("couldn't find start")?,
            end: end.context("couldn't find end")?,
        });
    }
}

pub fn part_1(input: &str) -> usize {
    let mut board: Board = input.parse().unwrap();
    print!("{:?}", board);
    let mut open: BinaryHeap<Node> = BinaryHeap::new();
    open.push(Node::new_root(board.start));
    let mut closed: Vec<Node> = vec![];

    'top: while let Some(q) = open.pop() {
        println!("q: {:?}", q);
        let successors = board.get_succesors(q);
        for suc in successors.iter() {
            println!("suc: {:?}", suc);
            if suc.loc == board.end {
                println!("Found the END!!!");
                board.set_parent(suc.loc, q.loc);
                break 'top;
            }
            if open
                .iter()
                .find(|n| n.loc == suc.loc && n.f <= suc.f)
                .map_or_else(|| false, |_| true)
                || closed
                    .iter()
                    .find(|n| n.loc == suc.loc && n.f <= suc.f)
                    .map_or_else(|| false, |_| true)
            {
                continue;
            }
            if suc.loc == (3, 4) {
                println!("I AM SETTING SUC 3,4 TO {:?}", q.loc);
            }
            board.set_parent(suc.loc, q.loc);
            open.push(*suc);
        }
        closed.push(q);
    }
    let mut node = board.get_parent(board.end);
    let mut count = 0;
    println!("starting search {:?}", node);
    while let Some(p) = node {
        println!("parent: {:?}", p);
        if p == board.start {
            return count + 1;
        }
        if count >= 50 {
            return 0;
        }
        count += 1;
        node = board.get_parent(p);
    }
    return 0;
}
pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}
