use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    solve(&chars, 4)
}


pub fn part_two(input: &str) -> Option<u32> {
    let chars: Vec<_> = input.chars().collect();
    solve(&chars, 14)
}

fn solve(chars: &[char], marker_len: u32) -> Option<u32> {
    for (win_id, w) in chars.windows(marker_len as usize).enumerate() {
        if !(0..marker_len)
            .cartesian_product(0..marker_len)
            .filter(|(x, y)| x != y)
            .any(|(x, y)| w[x as usize] == w[y as usize])
        {
            return Some(win_id as u32 + marker_len);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
