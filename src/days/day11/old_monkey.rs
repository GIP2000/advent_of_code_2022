use anyhow::{bail, Context, Result};
use std::str::FromStr;

pub struct OldMonkey {
    pub item_worry: Vec<f64>,
    pub operation: Box<dyn Fn(f64) -> f64>,
    pub test: Box<dyn Fn(f64) -> usize>,
    pub inspection_count: u128,
}
impl FromStr for OldMonkey {
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
            .map(|s| s.trim().parse::<f64>().ok().context("failed to parse item"))
            .collect::<Result<Vec<_>>>()?;
        let (_, op) = lines
            .next()
            .context("can't parse op")?
            .split_once("new = old ")
            .context("can't split op")?;

        let num: Option<f64> = op.trim()[1..].trim().parse().ok();

        let (_, test) = lines
            .next()
            .context("can't parse test")?
            .split_once("by ")
            .context("can't split test")?;
        let test = test.trim().parse::<f64>()?;
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

        let test = Box::new(move |worry| if worry % test == 0.0 { tr } else { fl });
        let operation: Box<dyn Fn(f64) -> f64> = match num {
            Some(num) => match &op.trim()[0..1] {
                "+" => Box::new(move |old| old + num),
                "*" => Box::new(move |old| old * num),
                _ => {
                    bail!("Invalid op")
                }
            },
            None => match &op.trim()[0..1] {
                "+" => Box::new(move |old| old + old),
                "*" => Box::new(move |old| old * old),
                _ => {
                    bail!("Invalid op")
                }
            },
        };

        Ok(Self {
            item_worry: items,
            test,
            operation,
            inspection_count: 0,
        })
    }
}
