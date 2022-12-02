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

    fn score(&self) -> u32 {
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

    fn score(&self) -> u32 {
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
            .map(|(opponent_move, my_move)| {
                my_move.score()
                    + (match (opponent_move, my_move) {
                        (Move::Rock, Move::Rock) => Outcome::Draw.score(),
                        (Move::Rock, Move::Paper) => Outcome::Win.score(),
                        (Move::Rock, Move::Scissors) => Outcome::Loss.score(),
                        (Move::Paper, Move::Rock) => Outcome::Loss.score(),
                        (Move::Paper, Move::Paper) => Outcome::Draw.score(),
                        (Move::Paper, Move::Scissors) => Outcome::Win.score(),
                        (Move::Scissors, Move::Rock) => Outcome::Win.score(),
                        (Move::Scissors, Move::Paper) => Outcome::Loss.score(),
                        (Move::Scissors, Move::Scissors) => Outcome::Draw.score(),
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
            .map(|(opponent_move, desired_outcome)| {
                desired_outcome.score()
                    + match (opponent_move, &desired_outcome) {
                        (Move::Rock, Outcome::Win) => Move::Paper.score(),
                        (Move::Rock, Outcome::Loss) => Move::Scissors.score(),
                        (Move::Rock, Outcome::Draw) => Move::Rock.score(),
                        (Move::Paper, Outcome::Win) => Move::Scissors.score(),
                        (Move::Paper, Outcome::Loss) => Move::Rock.score(),
                        (Move::Paper, Outcome::Draw) => Move::Paper.score(),
                        (Move::Scissors, Outcome::Win) => Move::Rock.score(),
                        (Move::Scissors, Outcome::Loss) => Move::Paper.score(),
                        (Move::Scissors, Outcome::Draw) => Move::Scissors.score(),
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
