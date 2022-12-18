use anyhow::{bail, Context, Result};
use std::{cmp::Ordering, str::FromStr};

#[derive(Clone, Debug)]
enum El {
    List(Vec<El>),
    Val(i32),
}

#[derive(Clone, Debug)]
struct Lists {
    right: El,
    left: El,
}
impl FromStr for El {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        use El::{List, Val};

        fn recursive_from_str(sl: &str) -> Result<(El, usize)> {
            let mut root = Vec::new();
            let mut num: Option<(usize, usize)> = None;
            let mut i = 0;
            while i < sl.len() {
                // println!("root: {:?}", root);
                let c = sl.chars().nth(i).context("Invalid index")?;
                match c {
                    '[' => {
                        // println!("new");
                        let (n_node, sk) = recursive_from_str(&sl[i + 1..])?;
                        root.push(n_node);
                        i += sk + 1;
                    }
                    ']' => {
                        // println!("end");
                        if let Some(num) = num {
                            let v = sl[num.0..num.1].parse::<i32>()?;
                            root.push(Val(v));
                        }
                        return Ok((List(root), i));
                    }
                    ',' => {
                        // println!("comma");
                        if !(i == 0 || sl.chars().nth(i - 1).unwrap() == ']') {
                            // println!("comma number");
                            if let Some(num) = num {
                                // println!("comma makes sense, num = {:?}", num);
                                let v = sl[num.0..num.1].parse::<i32>()?;
                                root.push(Val(v));
                                // println!("root post push: {:?}", root);
                            }
                        }
                        num = None;
                    }
                    _ => {
                        // println!("val {}", c);
                        match &mut num {
                            Some(num) => {
                                num.1 = i + 1;
                                // println!("num {:?}", num);
                            }
                            None => {
                                num = Some((i, i + 1));
                                // println!("num {:?}", num);
                            }
                        }
                        // println!("num after{:?}", num);
                    }
                };
                i += 1;
            }
            bail!("Invalid: expected ]");
        }
        let (root, _) = recursive_from_str(&s[1..])?;
        return Ok(root);
    }
}

impl FromStr for Lists {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().take(2).map(str::parse);
        return Ok(Self {
            left: lines.next().context("Can't find first list")??,
            right: lines.next().context("Can't find second list")??,
        });
    }
}

enum State {
    Good,
    Bad,
    Continue,
}

fn recursive_compare(lsts: &Lists) -> State {
    // println!("compare top: {lsts:?}");
    match (&lsts.left, &lsts.right) {
        (El::List(l_list), El::List(r_list)) => {
            // println!("l_list {:?}, r_list {:?}", l_list, r_list);
            for (l_val, r_val) in l_list.iter().zip(r_list) {
                // println!("l_val {:?}, r_val {:?}", l_val, r_val);
                match recursive_compare(&Lists {
                    left: l_val.clone(),
                    right: r_val.clone(),
                }) {
                    State::Good => {
                        return State::Good;
                    }
                    State::Bad => {
                        return State::Bad;
                    }
                    State::Continue => {
                        continue;
                    }
                }
            }
            if l_list.len() < r_list.len() {
                // println!("Good list");
                return State::Good;
            } else if l_list.len() == r_list.len() {
                // println!("Continue list");
                return State::Continue;
            } else {
                // println!("Bad list");
                return State::Bad;
            }
        }
        (El::Val(l_val), El::Val(r_val)) => {
            if l_val < r_val {
                // println!("Good val");
                return State::Good;
            } else if l_val == r_val {
                // println!("Continue val");
                return State::Continue;
            } else {
                // println!("Bad val");
                return State::Bad;
            }
        }
        (El::List(l_list), El::Val(r_val)) => recursive_compare(&Lists {
            left: El::List(l_list.clone()),
            right: El::List(vec![El::Val(*r_val)]),
        }),
        (El::Val(l_val), El::List(r_list)) => recursive_compare(&Lists {
            right: El::List(r_list.clone()),
            left: El::List(vec![El::Val(*l_val)]),
        }),
    }
}

pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, lsts)| {
            let lsts: Lists = lsts.parse().unwrap();
            // println!("comparing : {}", i + 1);
            match recursive_compare(&lsts) {
                State::Good => {
                    // println!("good: {}", i + 1);
                    return Some(i + 1);
                }
                State::Bad => None,
                State::Continue => None,
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    use El::{List, Val};
    let mut input = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect::<Result<Vec<El>>>()
        .expect("All inputs should be valid");
    input.push(List(vec![List(vec![Val(2)])]));
    input.push(List(vec![List(vec![Val(6)])]));
    input.sort_by(|a, b| {
        match recursive_compare(&Lists {
            left: a.clone(),
            right: b.clone(),
        }) {
            State::Good => Ordering::Less,
            State::Bad => Ordering::Greater,
            State::Continue => unreachable!("TF?"),
        }
    });
    let mut a = 0;
    let mut b = 0;
    for (i, v) in input.into_iter().enumerate() {
        if let List(o) = v {
            if o.len() != 1 {
                continue;
            }
            if let List(inn) = &o[0] {
                if inn.len() != 1 {
                    continue;
                }
                if let Val(x) = inn[0] {
                    if x == 2 {
                        a = i + 1;
                    }
                    if x == 6 {
                        b = i + 1;
                    }
                }
            }
        }
    }
    a * b
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 13);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 140);
    }
}
