pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut pack_sums: Vec<u32> = Vec::new();
    let mut cur_vec: Vec<u32> = Vec::new();

    for l in lines {
        if !l.is_empty() {
            cur_vec.push(l.parse::<u32>().ok()?);
        } else {
            pack_sums.push(cur_vec.iter().sum());
            cur_vec = Vec::new();
        }
    }

    pack_sums.iter().max().copied()

}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();

    let mut packs: Vec<Vec<u32>> = Vec::new();
    let mut cur_vec: Vec<u32> = Vec::new();

    for l in lines {
        if !l.is_empty() {
            cur_vec.push(l.parse::<u32>().ok()?);
        } else {
            packs.push(cur_vec.to_owned());
            cur_vec = Vec::new();
        }
    }

    let mut sums: Vec<u32> = packs.iter().map(|v| v.iter().sum()).collect();
    sums.sort_by(|a, b| b.cmp(a));
    Some(sums.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
