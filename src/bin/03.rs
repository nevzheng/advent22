pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (a, b): (&str, &str) = l.split_at(l.len() / 2);

                for x in a.bytes() {
                    for y in b.bytes() {
                        if x == y {
                            if x.is_ascii_lowercase() {
                                return x as u32 - 'a' as u32 + 1;
                            } else {
                                return x as u32 - 'A' as u32 + 1 + 26;
                            }
                        }
                    }
                }

                0u32
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();

    let mut score = 0;
    for v in lines.chunks(3) {
        let a = v[0];
        let b = v[1];
        let c = v[2];

        let m = find_match(a, b, c);
        if m.is_ascii_lowercase() {
            score += m as u32 - 'a' as u32 + 1;
        } else {
            score += m as u32 - 'A' as u32 + 1 + 26;
        }
    }

    Some(score)
}

fn find_match(a: &str, b: &str, c: &str) -> u8 {
    for x in a.bytes() {
        for y in b.bytes() {
            for z in c.bytes() {
                if x == y && x == z {
                    return x;
                }
            }
        }
    }
    0
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
}
