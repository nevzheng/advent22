#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn unique_items(sack: &str) -> u64 {
    sack.bytes()
        .map(|c| match c {
            b'a'..=b'z' => 1 + c - b'a',
            b'A'..=b'Z' => 1 + c - b'A',
            _ => unreachable!(),
        })
        .fold(0, |acc, n| acc | (1 << n))
}

pub fn part_one_bitwise(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(l, r)| [l, r].map(unique_items))
            .map(|[l, r]| u64::trailing_zeros(l & r))
            .sum(),
    )
}

pub fn part_two_bitwise(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .array_chunks::<3>()
            .map(|v| v.map(unique_items))
            .map(|[a, b, c]| a & b & c)
            .map(u64::trailing_zeros)
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (a, b): (&str, &str) = l.split_at(l.len() / 2);

                let a_set = a.bytes().collect::<HashSet<_>>();

                let c = b.bytes().find(|c| a_set.contains(c)).unwrap();

                score_match(c)
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    Some(
        lines
            .chunks(3)
            .map(|v| {
                let a = v[0];
                let b = v[1];
                let c = v[2];

                let m = find_match(a, b, c);
                score_match(m)
            })
            .sum::<u32>(),
    )
}

fn score_match(m: u8) -> u32 {
    if m.is_ascii_lowercase() {
        m as u32 - 'a' as u32 + 1
    } else {
        m as u32 - 'A' as u32 + 1 + 26
    }
}

fn find_match(a: &str, b: &str, c: &str) -> u8 {
    let a_set = a.bytes().collect::<HashSet<_>>();
    let b_set = b.bytes().collect::<HashSet<_>>();

    c.bytes()
        .find(|z| a_set.contains(z) && b_set.contains(z))
        .unwrap()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_part_one_bitwise() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one_bitwise(&input), Some(157));
    }

    #[test]
    fn test_part_two_bitwise() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two_bitwise(&input), Some(70));
    }
}
