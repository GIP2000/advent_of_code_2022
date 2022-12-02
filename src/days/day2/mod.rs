#[derive(Clone)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3,
}

impl From<&str> for RPS {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::ROCK,
            "B" => Self::PAPER,
            "C" => Self::SCISSORS,
            "X" => Self::ROCK,
            "Y" => Self::PAPER,
            "Z" => Self::SCISSORS,
            _ => unreachable!(),
        }
    }
}

impl RPS {
    fn into_score(&self) -> u32 {
        self.clone() as u32
    }

    fn win(&self) -> u32 {
        self.into_score() + 6
    }

    fn draw(&self) -> u32 {
        self.into_score() + 3
    }

    fn game(&self, op: &Self) -> u32 {
        match (self, op) {
            (RPS::ROCK, RPS::ROCK) => self.draw(),
            (RPS::ROCK, RPS::PAPER) => self.into_score(),
            (RPS::ROCK, RPS::SCISSORS) => self.win(),
            (RPS::PAPER, RPS::ROCK) => self.win(),
            (RPS::PAPER, RPS::PAPER) => self.draw(),
            (RPS::PAPER, RPS::SCISSORS) => self.into_score(),
            (RPS::SCISSORS, RPS::ROCK) => self.into_score(),
            (RPS::SCISSORS, RPS::PAPER) => self.win(),
            (RPS::SCISSORS, RPS::SCISSORS) => self.draw(),
        }
    }
}

pub fn part_1(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|game| !game.is_empty())
        .map(|game| {
            let (op, me) = game.split_once(" ").unwrap();
            let op = RPS::from(op);
            let me = RPS::from(me);
            me.game(&op)
        })
        .sum()
}

#[derive(Clone)]
enum Outcome {
    WIN = 6,
    LOSE = 0,
    DRAW = 3,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Self::LOSE,
            "Y" => Self::DRAW,
            "Z" => Self::WIN,
            _ => unreachable!(),
        }
    }
}

impl Outcome {
    fn into_score(&self, v: RPS) -> u32 {
        v as u32 + self.clone() as u32
    }

    fn game(&self, op: &RPS) -> u32 {
        match (self, op) {
            (Outcome::WIN, RPS::ROCK) => self.into_score(RPS::PAPER),
            (Outcome::WIN, RPS::PAPER) => self.into_score(RPS::SCISSORS),
            (Outcome::WIN, RPS::SCISSORS) => self.into_score(RPS::ROCK),
            (Outcome::LOSE, RPS::ROCK) => self.into_score(RPS::SCISSORS),
            (Outcome::LOSE, RPS::PAPER) => self.into_score(RPS::ROCK),
            (Outcome::LOSE, RPS::SCISSORS) => self.into_score(RPS::PAPER),
            (Outcome::DRAW, RPS::ROCK) => self.into_score(RPS::ROCK),
            (Outcome::DRAW, RPS::PAPER) => self.into_score(RPS::PAPER),
            (Outcome::DRAW, RPS::SCISSORS) => self.into_score(RPS::SCISSORS),
        }
    }
}

pub fn part_2(input: &str) -> u32 {
    input
        .split('\n')
        .filter(|game| !game.is_empty())
        .map(|game| {
            let (op, outcome) = game.split_once(" ").unwrap();
            let op = RPS::from(op);
            let outcome = Outcome::from(outcome);
            outcome.game(&op)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "A Y
B X
C Z";

    #[test]
    fn test() {
        assert_eq!(part_1(INPUT), 15);
        assert_eq!(part_2(INPUT), 12);
    }
}
