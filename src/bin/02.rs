enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from(c: &str) -> Move {
        match c {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => panic!("Invalid Value"),
        }
    }

    fn move_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn from(c: &str) -> Outcome {
        match c {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Invalid Value"),
        }
    }

    fn outcome_score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Loss => 0,
            Outcome::Draw => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(' ').unwrap();
                (Move::from(a), Move::from(b))
            })
            .map(|(opponent, me)| {
                me.move_score()
                    + (match (opponent, me) {
                        (Move::Rock, Move::Rock) => 3,
                        (Move::Rock, Move::Paper) => 6,
                        (Move::Rock, Move::Scissors) => 0,
                        (Move::Paper, Move::Rock) => 0,
                        (Move::Paper, Move::Paper) => 3,
                        (Move::Paper, Move::Scissors) => 6,
                        (Move::Scissors, Move::Rock) => 6,
                        (Move::Scissors, Move::Paper) => 0,
                        (Move::Scissors, Move::Scissors) => 3,
                    })
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(' ').unwrap();
                (Move::from(a), Outcome::from(b))
            })
            .map(|(opponent, me)| {
                me.outcome_score()
                    + match (opponent, &me) {
                        (Move::Rock, Outcome::Win) => Move::Paper.move_score(),
                        (Move::Rock, Outcome::Loss) => Move::Scissors.move_score(),
                        (Move::Rock, Outcome::Draw) => Move::Rock.move_score(),
                        (Move::Paper, Outcome::Win) => Move::Scissors.move_score(),
                        (Move::Paper, Outcome::Loss) => Move::Rock.move_score(),
                        (Move::Paper, Outcome::Draw) => Move::Paper.move_score(),
                        (Move::Scissors, Outcome::Win) => Move::Rock.move_score(),
                        (Move::Scissors, Outcome::Loss) => Move::Paper.move_score(),
                        (Move::Scissors, Outcome::Draw) => Move::Scissors.move_score(),
                    }
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
