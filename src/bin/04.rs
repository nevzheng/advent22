pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| {
                (
                    x.split('-')
                        .map(|a| a.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                    y.split('-')
                        .map(|b| b.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(x, y)| {
                ((x[0] <= y[0]) && (y[1] <= x[1])) || ((y[0] <= x[0]) && (x[1] <= y[1]))
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|(x, y)| {
                (
                    x.split('-')
                        .map(|a| a.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                    y.split('-')
                        .map(|b| b.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
            .filter(|(x, y)| {
                ((x[0]..=x[1]).contains(&y[0]))
                    || ((x[0]..=x[1]).contains(&y[1]))
                    || ((y[0]..=y[1]).contains(&x[0]))
                    || ((y[0]..=y[1]).contains(&x[1]))
            })
            .count()
            .try_into()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
