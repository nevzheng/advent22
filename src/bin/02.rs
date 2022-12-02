const OP_ROCK: &str = "A";
const OP_PAPER: &str = "B";
const OP_SCISSORS: &str = "C";

const MY_ROCK: &str = "X";
const MY_PAPER: &str = "Y";
const MY_SCISSORS: &str = "Z";

const MY_LOSS: &str = "X";
const MY_DRAW: &str = "Y";
const MY_WIN: &str = "Z";

pub fn part_one(input: &str) -> Option<u32> {
    let strategy_guide: Vec<(&str, &str)> =
        input.lines().map(|l| l.split_once(' ').unwrap()).collect();

    let mut score = 0;

    for (opponent_play, our_play) in strategy_guide {
        match (opponent_play, our_play) {
            (OP_ROCK, MY_ROCK) => score += 1 + 3,
            (OP_ROCK, MY_PAPER) => score += 2 + 6,
            (OP_ROCK, MY_SCISSORS) => score += 3 + 0,

            (OP_PAPER, MY_ROCK) => score += 1 + 0,
            (OP_PAPER, MY_PAPER) => score += 2 + 3,
            (OP_PAPER, MY_SCISSORS) => score += 3 + 6,

            (OP_SCISSORS, MY_ROCK) => score += 1 + 6,
            (OP_SCISSORS, MY_PAPER) => score += 2 + 0,
            (OP_SCISSORS, MY_SCISSORS) => score += 3 + 3,
            _ => panic!("Incorrect Input."),
        }
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let strategy_guide: Vec<(&str, &str)> =
        input.lines().map(|l| l.split_once(' ').unwrap()).collect();

    let mut score = 0;

    for (opponent_play, our_play) in strategy_guide {
        match (opponent_play, our_play) {
            (OP_ROCK, MY_WIN) => score += 6 + 2,
            (OP_PAPER, MY_WIN) => score += 6 + 3,
            (OP_SCISSORS, MY_WIN) => score += 6 + 1,

            (OP_ROCK, MY_LOSS) => score += 3,
            (OP_PAPER, MY_LOSS) => score += 1,
            (OP_SCISSORS, MY_LOSS) => score += 2,

            (OP_ROCK, MY_DRAW) => score += 3 + 1,
            (OP_PAPER, MY_DRAW) => score += 3 + 2,
            (OP_SCISSORS, MY_DRAW) => score += 3 + 3,

            _ => panic!("Incorrect Input."),
        }
    }

    Some(score)
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
