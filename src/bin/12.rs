use std::collections::{HashSet, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, height, width, start, dest) = read_grid(input);

    let mut queue: VecDeque<(isize, isize)> = VecDeque::from([start]);
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut depth: u32 = 0;

    let deltas = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    while !queue.is_empty() {
        let level_size = queue.len();
        for _ in 0..level_size {
            let (cx, cy) = queue.pop_back().unwrap();
            if visited.contains(&(cx, cy)) {
                continue;
            }

            visited.insert((cx, cy));

            if (cx, cy) == dest {
                return Some(depth);
            }

            for (dx, dy) in deltas.iter() {
                let (nx, ny) = (cx + dx, cy + dy);

                // Bounds Check
                if nx < 0 || nx >= height || ny < 0 || ny >= width {
                    continue;
                }

                // If start, add all neighbors.
                // Else Check that its increaseing by 1.
                if (grid[cx as usize][cy as usize] as u8) + 1
                    >= (grid[nx as usize][ny as usize] as u8)
                {
                    queue.push_front((nx, ny));
                }
            }
        }
        depth += 1;
    }

    None
}

fn read_grid(input: &str) -> (Vec<Vec<char>>, isize, isize, (isize, isize), (isize, isize)) {
    let mut grid: Vec<_> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();
    let height = grid.len() as isize;
    let width = grid.first().unwrap().len() as isize;
    let start = find_point('S', height, width, &grid).unwrap();
    let dest = find_point('E', height, width, &grid).unwrap();

    print_grid(&grid);
    grid[start.0 as usize][start.1 as usize] = 'a';
    grid[dest.0 as usize][dest.1 as usize] = 'z';
    println!("=========");
    print_grid(&grid);

    (grid, height, width, start, dest)
}

fn print_grid(grid: &[Vec<char>]) {
    for l in grid.iter() {
        for c in l.iter() {
            print!("{c}")
        }
        println!()
    }
}

fn find_point(c: char, height: isize, width: isize, grid: &[Vec<char>]) -> Option<(isize, isize)> {
    for i in 0..height {
        for j in 0..width {
            if grid[i as usize][j as usize] == c {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
