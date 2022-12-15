use std::cmp::max;

pub fn part_one(input: &str) -> Option<i64> {
    // A jeading has format: (sx, sy, bx, by)
    let readings = read_input(input);
    let row = 2000000;

    let mut ranges = get_empty_ranges(&readings, row);
    let ranges = merge_overlapping_intervals(&mut ranges);
    Some(ranges.iter().map(|(a, b)| b - a).sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let readings = read_input(input);

    for &(x, y, dx, dy) in readings.iter() {
        let dist = (x - dx).abs() + (y - dy).abs();
        for (dir_x, dir_y) in [(-1, -1), (-1, 1), (1, -1), (1, 1)] {
            for d in 0..dist {
                let bx = x + dir_x * d;
                let by = y + dir_y * (dist + 1 - d);
                if bx < 0 || by < 0 || bx > 4000000 || by > 4000000 {
                    break;
                }
                let found = readings.iter().all(|&(x, y, dx, dy)| {
                    let d1 = (x - dx).abs() + (y - dy).abs();
                    let d2 = (bx - x).abs() + (by - y).abs();
                    d2 >= d1
                });
                if found {
                    return Some(bx * 4000000 + by);
                }
            }
        }
    }

    None
}

fn read_input(input: &str) -> Vec<(i64, i64, i64, i64)> {
    input
        .lines()
        .map(|l| {
            sscanf::sscanf!(
                l,
                "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
            )
            .unwrap()
        })
        .collect()
}

fn merge_overlapping_intervals(ranges: &mut [(i64, i64)]) -> Vec<(i64, i64)> {
    ranges.sort();

    let mut result: Vec<(i64, i64)> = Vec::new();
    result.push(ranges[0]);

    for i in 1..ranges.len() {
        let cur = ranges[i];

        let j = result.len() - 1;
        if cur.0 >= result[j].0 && cur.0 <= result[j].1 {
            result[j].1 = max(cur.1, result[j].1);
        } else {
            result.push(cur);
        }
    }

    result
}

fn get_empty_ranges(readings: &[(i64, i64, i64, i64)], row: i64) -> Vec<(i64, i64)> {
    // ranges where beacons can't exist
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    // for each reading determine the intersection of`row` with the bounding box.
    for (sx, sy, bx, by) in readings {
        let distance = (bx - sx).abs() + (by - sy).abs();
        // The bounding box intersects with `row`
        if (row - sy).abs() < distance {
            // Compute the range for which the bounding box intersects `row`.
            let k = distance - (row - sy).abs();
            ranges.push((sx - k, sx + k));
        }
    }
    ranges
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);

        let readings = read_input(&input);
        let row = 10;

        let mut ranges = get_empty_ranges(&readings, row);
        let ranges = merge_overlapping_intervals(&mut ranges);
        let result = Some(ranges.iter().map(|(a, b)| b - a).sum::<i64>());

        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
