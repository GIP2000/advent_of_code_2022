use anyhow::{bail, Context, Result};
use std::str::FromStr;
mod old_monkey;
use old_monkey::OldMonkey;
#[derive(Clone, Debug)]
enum Chain {
    Add(usize),
    Mul(usize),
    MulSelf,
}

#[derive(Debug)]
struct Monkey {
    item_worry_index: Vec<usize>,
    operation: Chain,
    test: usize,
    tr: usize,
    fl: usize,
    inspection_count: u128,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let (_, items) = lines
            .next()
            .context("can't find items")?
            .split_once(":")
            .context("can't split items")?;
        let items = items
            .split(",")
            .map(|s| {
                s.trim()
                    .parse::<usize>()
                    .ok()
                    .context(format!("Can't parse number *{}*", s))
            })
            .collect::<Result<Vec<_>>>()?;
        let (_, op) = lines
            .next()
            .context("can't parse op")?
            .split_once("new = old ")
            .context("can't split op")?;

        let num: Option<usize> = op.trim()[1..].trim().parse().ok();

        let (_, test) = lines
            .next()
            .context("can't parse test")?
            .split_once("by ")
            .context("can't split test")?;
        let test = test.trim().parse::<usize>()?;
        let (_, tr) = lines
            .next()
            .context("can't parse true")?
            .split_once("monkey ")
            .context("can't split true")?;
        let tr = tr.trim().parse::<usize>()?;
        let (_, fl) = lines
            .next()
            .context("can't parse false")?
            .split_once("monkey ")
            .context("can't split false")?;
        let fl = fl.trim().parse::<usize>()?;

        let operation: Chain = match num {
            Some(num) => match &op.trim()[0..1] {
                "+" => Chain::Add(num),
                "*" => Chain::Mul(num),
                _ => {
                    bail!("Invalid op")
                }
            },
            None => Chain::MulSelf,
        };

        Ok(Self {
            item_worry_index: items,
            test,
            fl,
            tr,
            operation,
            inspection_count: 0,
        })
    }
}

pub fn part_1(input: &str, calm: u128, rounds: usize) -> u128 {
    let mut monkeys = input
        .split("\n\n")
        .map(|s| s.parse::<OldMonkey>().ok().context("Invalid Monkey"))
        .collect::<Result<Vec<_>>>()
        .unwrap();
    for _round in 0..rounds {
        for m in 0..monkeys.len() {
            let mut to_remove = vec![];
            for i in 0..monkeys[m].item_worry.len() {
                monkeys[m].item_worry[i] =
                    ((monkeys[m].operation)(monkeys[m].item_worry[i]) as u128 / calm) as f64;
                monkeys[m].inspection_count += 1;
                to_remove.push(i);
                let pos = (monkeys[m].test)(monkeys[m].item_worry[i]);
                let val = monkeys[m].item_worry[i];
                monkeys[pos].item_worry.push(val);
            }
            for (dx, i) in to_remove.into_iter().enumerate() {
                monkeys[m].item_worry.remove(i - dx);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count)
}

pub fn part_2(input: &str) -> u128 {
    // collect monkeysj
    let mut monkeys = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>())
        .collect::<Result<Vec<_>>>()
        .unwrap();
    // build matrix
    let mut counter = 0;
    let mut items = vec![];
    for m in monkeys.iter_mut() {
        for v in m.item_worry_index.iter_mut() {
            items.push(*v);
            *v = counter;
            counter += 1;
        }
    }
    let mut matrix: Vec<Vec<usize>> = vec![items; monkeys.len()];
    for (mi, mv) in matrix.iter_mut().enumerate() {
        for mvv in mv.iter_mut() {
            *mvv = *mvv % monkeys[mi].test;
        }
    }

    for _round in 0..10000 {
        for m in 0..monkeys.len() {
            let mut to_remove = vec![];
            for worry_index in 0..monkeys[m].item_worry_index.len() {
                to_remove.push(worry_index);
                let new_val_index = monkeys[m].item_worry_index[worry_index];
                for (mi, monkey_matrix) in matrix.iter_mut().enumerate() {
                    match monkeys[m].operation {
                        Chain::Add(v) => {
                            monkey_matrix[new_val_index] = (monkey_matrix[new_val_index]
                                + v % monkeys[mi].test)
                                % monkeys[mi].test
                        }
                        Chain::Mul(v) => {
                            monkey_matrix[new_val_index] =
                                (monkey_matrix[new_val_index] * v) % monkeys[mi].test
                        }
                        Chain::MulSelf => {
                            monkey_matrix[new_val_index] = (monkey_matrix[new_val_index]
                                * monkey_matrix[new_val_index])
                                % monkeys[mi].test
                        }
                    }
                }
                monkeys[m].inspection_count += 1;
                let new_pos = if matrix[m][new_val_index] == 0 {
                    monkeys[m].tr
                } else {
                    monkeys[m].fl
                };
                monkeys[new_pos].item_worry_index.push(new_val_index);
            }

            for (x, i) in to_remove.into_iter().enumerate() {
                monkeys[m].item_worry_index.remove(i - x);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());
    monkeys
        .iter()
        .take(2)
        .fold(1, |acc, m| acc * m.inspection_count)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT, 3, 20), 10605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2713310158);
    }
}
