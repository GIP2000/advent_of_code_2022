use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

struct Valve {
    flow_rate: u32,
    tunnels: Vec<Rc<RefCell<Valve>>>,
    name: String,
}

impl Debug for Valve {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Valve {} has flow rate = {}; tunnels lead to valves {}",
            self.name,
            self.flow_rate,
            self.tunnels
                .iter()
                .map(|t| { t.borrow().name.clone() })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

fn parse_graph(input: &str) -> Result<Vec<Rc<RefCell<Valve>>>> {
    let mut map: HashMap<String, Rc<RefCell<Valve>>> = HashMap::new();

    let mut verts = input
        .lines()
        .map(|ln| {
            let name = ln.chars().skip(6).take(2).collect::<String>();
            let (_, rate) = ln.split_once("rate=").context("Invalid Input string")?;
            let (rate, _) = rate.split_once(";").context("Invalid Input string")?;
            let rate = rate.parse::<u32>()?;
            let (_, tunnels) = match ln.split_once("valves ") {
                Some(v) => v,
                None => ln
                    .split_once("valve ")
                    .context("No Tunnels found for Valve")?,
            };
            let tunnels = tunnels
                .split(",")
                .map(|t| {
                    let name = t.trim().to_string();
                    match map.get(&name) {
                        Some(v) => {
                            return v.clone();
                        }
                        None => {
                            let val = Rc::new(RefCell::new(Valve {
                                flow_rate: 0,
                                tunnels: vec![],
                                name: name.clone(),
                            }));
                            map.insert(name, val.clone());
                            return val;
                        }
                    }
                })
                .collect::<Vec<_>>();
            match map.get(&name) {
                Some(v) => {
                    v.borrow_mut().flow_rate = rate;
                    v.borrow_mut().tunnels = tunnels;
                    return Ok(v.clone());
                }
                None => {
                    let val = Rc::new(RefCell::new(Valve {
                        flow_rate: rate,
                        tunnels,
                        name: name.clone(),
                    }));
                    map.insert(name, val.clone());
                    return Ok(val);
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;

    verts.sort_by(|a, b| a.borrow().name.cmp(&b.borrow().name));
    return Ok(verts);
}

pub fn part_1(input: &str) -> usize {
    let verts = parse_graph(input).unwrap();
    println!("{:?}", verts);
    0
}
pub fn part_2(input: &str) -> usize {
    let verts = parse_graph(input).unwrap();
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 1651);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 0);
    }
}
