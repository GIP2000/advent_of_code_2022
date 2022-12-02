trait Game {
    fn game(&self, op: &RPS) -> u32;
}

#[derive(Clone)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl std::str::FromStr for RPS {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::ROCK),
            "B" => Ok(Self::PAPER),
            "C" => Ok(Self::SCISSORS),
            "X" => Ok(Self::ROCK),
            "Y" => Ok(Self::PAPER),
            "Z" => Ok(Self::SCISSORS),
            _ => Err(()),
        }
    }
}

impl Game for RPS {
    fn game(&self, op: &Self) -> u32 {
        (match (self, op) {
            (RPS::ROCK, RPS::ROCK) => Outcome::DRAW,
            (RPS::ROCK, RPS::PAPER) => Outcome::LOSE,
            (RPS::ROCK, RPS::SCISSORS) => Outcome::WIN,
            (RPS::PAPER, RPS::ROCK) => Outcome::WIN,
            (RPS::PAPER, RPS::PAPER) => Outcome::DRAW,
            (RPS::PAPER, RPS::SCISSORS) => Outcome::LOSE,
            (RPS::SCISSORS, RPS::ROCK) => Outcome::LOSE,
            (RPS::SCISSORS, RPS::PAPER) => Outcome::WIN,
            (RPS::SCISSORS, RPS::SCISSORS) => Outcome::DRAW,
        } as u32)
            + self.clone() as u32
    }
}

#[derive(Clone)]
enum Outcome {
    WIN = 6,
    LOSE = 0,
    DRAW = 3,
}

impl std::str::FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::LOSE),
            "Y" => Ok(Self::DRAW),
            "Z" => Ok(Self::WIN),
            _ => Err(()),
        }
    }
}

impl Game for Outcome {
    fn game(&self, op: &RPS) -> u32 {
        (match (self, op) {
            (Outcome::WIN, RPS::ROCK) => RPS::PAPER,
            (Outcome::WIN, RPS::PAPER) => RPS::SCISSORS,
            (Outcome::WIN, RPS::SCISSORS) => RPS::ROCK,
            (Outcome::LOSE, RPS::ROCK) => RPS::SCISSORS,
            (Outcome::LOSE, RPS::PAPER) => RPS::ROCK,
            (Outcome::LOSE, RPS::SCISSORS) => RPS::PAPER,
            (Outcome::DRAW, RPS::ROCK) => RPS::ROCK,
            (Outcome::DRAW, RPS::PAPER) => RPS::PAPER,
            (Outcome::DRAW, RPS::SCISSORS) => RPS::SCISSORS,
        } as u32)
            + self.clone() as u32
    }
}

fn do_day2<T>(input: &str) -> u32
where
    T: Game + std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|game| {
            let (op, outcome) = game.split_once(" ").unwrap();
            let op = op.parse::<RPS>().unwrap();
            let outcome = outcome.parse::<T>().unwrap();
            outcome.game(&op)
        })
        .sum()
}

pub fn part_1(input: &str) -> u32 {
    do_day2::<RPS>(input)
}

pub fn part_2(input: &str) -> u32 {
    do_day2::<Outcome>(input)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = "A Y\nB X\nC Z";
    #[test]
    fn test() {
        assert_eq!(part_1(INPUT), 15);
        assert_eq!(part_2(INPUT), 12);
    }
}
