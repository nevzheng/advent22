use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    let head_moves: Vec<(char, i32)> = input
        .lines()
        .map(|l| sscanf::scanf!(l, "{char} {i32}").unwrap())
        .collect();

    // Initial Positions.
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    for (direction, count) in head_moves {
        let dir = get_displacement(direction);
        for _ in 0..count {
            head = (head.0 + dir.0, head.1 + dir.1);

            let relative_pos = (head.0 - tail.0, head.1 - tail.1);

            let tail_delta = compute_delta(relative_pos);

            tail = (tail.0 + tail_delta.0, tail.1 + tail_delta.1);
            tail_visited.insert(tail);
        }
    }
    Some(tail_visited.len() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let head_moves: Vec<(char, i32)> = input
        .lines()
        .map(|l| sscanf::scanf!(l, "{char} {i32}").unwrap())
        .collect();

    let mut rope = vec![(0, 0); 10];

    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();

    for (direction, count) in head_moves {
        let dir = get_displacement(direction);
        for _ in 0..count {
            // Move the head.
            rope[0] = (rope[0].0 + dir.0, rope[0].1 + dir.1);

            // Update Rope
            for i in 1..rope.len() {
                let relative_pos = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                let delta = compute_delta(relative_pos);
                rope[i] = (rope[i].0 + delta.0, rope[i].1 + delta.1);
            }
            tail_visited.insert(rope.last().unwrap().to_owned());
        }
    }
    Some(tail_visited.len() as i32)
}

fn compute_delta(relative_pos: (i32, i32)) -> (i32, i32) {
    match relative_pos {
        // Adjacent to Head
        (0, 0) | (0, 1) | (0, -1) | (1, 0) | (-1, 0) | (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => {
            (0, 0)
        }
        // Orthonal to Head, Follow.
        (0, -2) => (0, -1),
        (0, 2) => (0, 1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        // Tail Moves Diagonally to follow head.
        (1, 2) | (2, 1) => (1, 1),
        (-1, -2) | (-2, -1) => (-1, -1),
        (2, -1) | (1, -2) => (1, -1),
        (-1, 2) | (-2, 1) => (-1, 1),
        // Only encountered for longer ropes.
        (2, 2) => (1, 1),
        (-2,-2) => (-1,-1),
        (-2, 2) => (-1, 1), 
        (2, -2) => (1,-1),
        _ => panic!("unexpected relative_pos: {:?}", relative_pos),
    }
}

fn get_displacement(direction: char) -> (i32, i32) {
    match direction {
        'R' => (0, 1),
        'L' => (0, -1),
        'U' => (1, 0),
        'D' => (-1, 0),
        _ => panic!("invalid direction"),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("src/examples/09_2.txt").unwrap();
        assert_eq!(part_two(&input), Some(36));
    }
}
