use anyhow::{Context, Result};
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
    rc::Rc,
};

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

fn parse_graph(input: &str) -> Result<HashMap<String, Rc<RefCell<Valve>>>> {
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
    return Ok(map);
}

fn max_flow(
    verts: &HashMap<String, Rc<RefCell<Valve>>>,
    dp: &mut HashMap<(String, String, u32), u32>,
    cur: String,
    opened: &HashSet<String>,
    min_left: u32,
) -> u32 {
    // check for dp
    let mut opened_str = opened.iter().cloned().collect::<Vec<_>>();
    opened_str.sort();
    let opened_str = opened_str.join("");
    let dp_key = (cur.clone(), opened_str, min_left);
    if let Some(&v) = dp.get(&dp_key) {
        return v;
    }
    // base case
    if min_left <= 0 {
        return 0;
    }

    let mut best = 0;
    let val = (min_left - 1) * verts.get(&cur).unwrap().borrow().flow_rate;
    for adj in verts
        .get(&cur)
        .unwrap()
        .borrow()
        .tunnels
        .iter()
        .map(|adj| adj.borrow())
    {
        if val != 0 && !opened.contains(&cur) {
            let mut cur_opened = opened.clone();
            cur_opened.insert(cur.clone());
            best = std::cmp::max(
                best,
                val + max_flow(verts, dp, adj.name.clone(), &cur_opened, min_left - 2),
            );
        }
        best = std::cmp::max(
            best,
            max_flow(verts, dp, adj.name.clone(), &opened, min_left - 1),
        );
    }
    dp.insert(dp_key, best);
    return best;
}

fn max_flow_2(
    verts: &HashMap<String, Rc<RefCell<Valve>>>,
    dp: &mut HashMap<(String, String, u32), u32>,
    cur: String,
    opened: &HashSet<String>,
    best_path: &mut Vec<HashSet<String>>,
    min_left: u32,
) -> u32 {
    // check for dp
    let mut opened_str = opened.iter().cloned().collect::<Vec<_>>();
    opened_str.sort();
    let opened_str = opened_str.join("");
    let dp_key = (cur.clone(), opened_str, min_left);
    if let Some(&v) = dp.get(&dp_key) {
        return v;
    }
    // base case
    if min_left <= 0 {
        return 0;
    }

    let mut best = 0;
    let val = (min_left - 1) * verts.get(&cur).unwrap().borrow().flow_rate;
    for adj in verts
        .get(&cur)
        .unwrap()
        .borrow()
        .tunnels
        .iter()
        .map(|adj| adj.borrow())
    {
        if val != 0 && !opened.contains(&cur) {
            let mut cur_opened = opened.clone();
            cur_opened.insert(cur.clone());
            let mut pot_opened = cur_opened.clone();
            let val_add = val
                + max_flow_2(
                    verts,
                    dp,
                    adj.name.clone(),
                    &cur_opened,
                    best_path,
                    min_left - 2,
                );
            best = std::cmp::max(best, val_add);
            // if val_add > best {
            //     best = val_add;
            // }
        }
        let mut pot_opened = opened.clone();
        let val_no_add = max_flow_2(
            verts,
            dp,
            adj.name.clone(),
            &opened,
            best_path,
            min_left - 1,
        );
        best = std::cmp::max(best, val_no_add);
    }
    dp.insert(dp_key, best);
    return best;
}

pub fn part_1(input: &str) -> u32 {
    let verts = parse_graph(input).unwrap();
    return max_flow_2(
        &verts,
        &mut HashMap::new(),
        "AA".to_string(),
        &HashSet::new(),
        &mut HashSet::new(),
        30,
    );
}
pub fn part_2(input: &str) -> u32 {
    let verts = parse_graph(input).unwrap();
    let mut best = HashSet::new();
    let mut dp = HashMap::new();
    let val = max_flow_2(
        &verts,
        &mut dp,
        "AA".to_string(),
        &HashSet::new(),
        &mut best,
        26,
    );
    println!("val: {val} best: {best:?}");
    let mut best_2 = HashSet::new();
    let val2 = max_flow_2(&verts, &mut dp, "AA".to_string(), &best, &mut best_2, 26);
    println!("val2 {val2}, best_2: {best_2:?}");
    val2 + val
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
        assert_eq!(part_2(INPUT), 1707);
    }
}
