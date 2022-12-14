use anyhow::{Context, Result};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::{ops::Index, str::FromStr};

type Cords = (usize, usize);

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Node {
    loc: Cords,
    g: u32,
    h: u32,
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
        other.f().cmp(&self.f())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    #[allow(dead_code)]
    pub fn new(loc: Cords, g: u32, h: u32) -> Self {
        Self { loc, g, h }
    }

    pub fn new_root(loc: Cords) -> Self {
        Self { loc, g: 0, h: 0 }
    }

    fn f(&self) -> u32 {
        self.g + self.h
    }
}

#[derive(Debug, Clone)]
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
                        new_node.g = q_node.g + 1;
                        // new_node.h = 0;
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

fn a_star(mut board: Board, found: Option<&mut HashSet<Cords>>) -> Option<usize> {
    // println!("{:?}", board);
    let mut open: Vec<Node> = vec![Node::new_root(board.start)];
    let mut closed: Vec<Node> = vec![];
    let mut has_path = false;

    'top: while let Some(q) = open.pop() {
        // println!("  f(q): {:?}", q.f());
        let successors = board.get_succesors(q);
        for suc in successors.iter() {
            // println!("    suc: {:?}", suc);
            if suc.loc == board.end {
                println!("Found the END!!!");
                board.set_parent(suc.loc, q.loc);
                has_path = true;
                break 'top;
            }
            if closed
                .iter()
                .find(|n| n.loc == suc.loc)
                .map_or_else(|| false, |_| true)
            {
                continue;
            }
            if let Some(n) = open
                .iter_mut()
                .find(|n| n.loc == suc.loc && n.f() <= suc.f())
            {
                n.g = suc.g;
                n.h = suc.h;
            } else {
                open.push(*suc);
            }
            board.set_parent(suc.loc, q.loc);
        }
        closed.push(q);
        open.sort_by(|a, b| a.cmp(b));
        // println!("o: {:?}", open);
    }
    if !has_path {
        return None;
    }
    let mut node = board.get_parent(board.end);
    let mut any_start = false;
    if let Some(found) = found {
        any_start = true;
        while let Some(p) = node {
            if board[p] == 0 {
                found.insert(p);
            }
            node = board.get_parent(p);
        }
        node = board.get_parent(board.end);
    }
    let mut count = 0;
    // println!("starting search {:?} until {:?}", node, board.start);
    // print!("{:?} -> ", board.end);
    while let Some(p) = node {
        count += 1;
        if (any_start && board[p] == 0) || (!any_start && p == board.start) {
            println!("END_found = {:?}", p);
            return Some(count);
        }
        // print!("{:?} -> ", p);
        node = board.get_parent(p);
    }
    println!("no path found");
    return None;
}

pub fn part_1(input: &str) -> usize {
    let board: Board = input.parse().unwrap();
    return a_star(board, None).unwrap();
}
pub fn part_2(input: &str) -> usize {
    let board: Board = input.parse().unwrap();
    let mut min_val = usize::MAX;
    let mut found = HashSet::new();

    for y in 0..board.map.len() {
        for x in 0..board.map[y].len() {
            if board.map[y][x].0 != 0 || found.contains(&(y, x)) {
                continue;
            }
            let mut b = board.clone();
            b.start = (y, x);
            let cur = a_star(b, Some(&mut found));
            if let None = cur {
                continue;
            }
            let cur = cur.unwrap();
            println!("found: {}, cur: {}", found.len(), cur);
            if cur < min_val {
                min_val = cur;
            }
        }
    }
    return min_val;
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[ignore]
    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 31);
    }
    #[ignore]
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 29);
    }
}
