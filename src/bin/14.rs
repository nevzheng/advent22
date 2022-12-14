use std::{
    cmp::{max, min},
    collections::HashSet,
};

const START_POINT: (i32, i32) = (500, 0);

pub fn part_one(input: &str) -> Option<i32> {
    // Read all line segments into a map of points.
    let rocks: HashSet<(i32, i32)> = read_input(input);
    // Points where sand has accumulated.
    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    // Find the min bound for the map.
    let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

    while let Some(stop_point) = simulate(&rocks, max_y, false, &sand) {
        sand.insert(stop_point);
    }

    Some(sand.len() as i32)
}

pub fn part_two(input: &str) -> Option<i32> {
    // Read all line segments into a map of points.
    let rocks: HashSet<(i32, i32)> = read_input(input);
    // Points where sand has accumulated.
    let mut sand: HashSet<(i32, i32)> = HashSet::new();

    // Find the min bound for the map.
    let max_y = *rocks.iter().map(|(_, y)| y).max().unwrap();

    while let Some(stop_point) = simulate(&rocks, max_y, true, &sand) {
        sand.insert(stop_point);
        if stop_point == START_POINT {
            break;
        }
    }

    Some(sand.len() as i32)
}

fn simulate(
    rocks: &HashSet<(i32, i32)>,
    max_y: i32,
    use_floor: bool,
    sand: &HashSet<(i32, i32)>,
) -> Option<(i32, i32)> {
    let start = START_POINT;
    let mut grain = start;

    while grain.1 <= max_y {
        let down = (grain.0, grain.1 + 1);
        let down_left = (grain.0 - 1, grain.1 + 1);
        let down_right = (grain.0 + 1, grain.1 + 1);

        if !rocks.contains(&down) && !sand.contains(&down) {
            grain = down;
        } else if !rocks.contains(&down_left) && !sand.contains(&down_left) {
            grain = down_left;
        } else if !rocks.contains(&down_right) && !sand.contains(&down_right) {
            grain = down_right;
        } else {
            return Some(grain);
        }
    }

    if use_floor {
        Some(grain)
    } else {
        None
    }
}

fn read_input(input: &str) -> HashSet<(i32, i32)> {
    let mut rocks: HashSet<(i32, i32)> = HashSet::new();

    for l in input.lines() {
        let points = read_points(l);
        for p in draw_line_segments(points) {
            rocks.insert(p);
        }
    }

    rocks
}

fn draw_line_segments(points: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut v: Vec<(i32, i32)> = Vec::new();
    for w in points.windows(2) {
        let ((sx, sy), (ex, ey)) = (w[0], w[1]);
        // Horizontal Line
        if sx == ex {
            let mn = min(sy, ey);
            let mx = max(sy, ey);
            for j in mn..=mx {
                v.push((sx, j));
            }
        } else {
            // Vertical Line
            let mn = min(sx, ex);
            let mx = max(sx, ex);
            for i in mn..=mx {
                v.push((i, sy));
            }
        }
    }
    v
}

fn read_points(line: &str) -> Vec<(i32, i32)> {
    line.split(" -> ")
        .map(|s| {
            let (xs, ys) = s.split_once(',').unwrap();
            (xs.parse::<i32>().unwrap(), ys.parse::<i32>().unwrap())
        })
        .collect::<Vec<(i32, i32)>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
