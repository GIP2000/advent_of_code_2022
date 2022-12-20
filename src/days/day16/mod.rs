pub fn part_1(input: &str) -> usize {
    0
}
pub fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
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
