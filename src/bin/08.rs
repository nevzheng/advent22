use std::cmp::max;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = read_grid(input);
    let height = grid.len();
    let width = grid.first().unwrap().len();

    let mut visible = vec![vec![false; width]; height];

    mark_edges(height, &mut visible, width);

    // For Each Row
    for i in 1..(height - 1) {
        scan_right(&grid, i, width, &mut visible);
        scan_left(&grid, i, width, &mut visible);
    }

    // For Each Column
    for j in 1..(width - 1) {
        scan_down(&grid, j, height, &mut visible);
        scan_up(&grid, height, j, &mut visible);
    }

    Some(
        visible
            .into_iter()
            .map(|v| v.into_iter().filter(|&x| x).count() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = read_grid(input);
    let height = grid.len();
    let width = grid.first().unwrap().len();

    (0..height)
        .cartesian_product(0..width)
        .map(|(i, j)| find_scenic_score(&grid, i, j, height, width))
        .max()
}

fn mark_edges(height: usize, visible: &mut Vec<Vec<bool>>, width: usize) {
    // Mark Left and Right Columns
    for i in 0..height {
        visible[i][0] = true;
        visible[i][width - 1] = true;
    }
    // Mark Top and Bottom Rows
    for j in 0..width {
        visible[0][j] = true;
        visible[height - 1][j] = true;
    }
}

fn scan_up(grid: &Vec<Vec<u8>>, height: usize, j: usize, visible: &mut Vec<Vec<bool>>) {
    let mut mx = grid[height - 1][j];
    // Bottom to Top
    for i in (1..(height - 1)).rev() {
        let cur = grid[i][j];
        visible[i][j] = visible[i][j] || cur > mx;
        mx = max(mx, cur);
    }
}

fn scan_down(grid: &Vec<Vec<u8>>, j: usize, height: usize, visible: &mut Vec<Vec<bool>>) {
    let mut mx = grid[0][j];
    // Top to Bottom
    for i in 1..(height - 1) {
        let cur = grid[i][j];
        visible[i][j] = visible[i][j] || cur > mx;
        mx = max(mx, cur);
    }
}

fn scan_left(grid: &Vec<Vec<u8>>, i: usize, width: usize, visible: &mut Vec<Vec<bool>>) {
    // Right to Left
    let mut mx = grid[i][width - 1];
    for j in (1..(width - 1)).rev() {
        let cur = grid[i][j];
        visible[i][j] = visible[i][j] || cur > mx;
        mx = max(mx, cur);
    }
}

fn scan_right(grid: &Vec<Vec<u8>>, i: usize, width: usize, visible: &mut Vec<Vec<bool>>) {
    // Left to Right
    let mut mx = grid[i][0];
    for j in 1..(width - 1) {
        let cur = grid[i][j];
        visible[i][j] = visible[i][j] || cur > mx;
        mx = max(mx, cur);
    }
}

fn find_scenic_score(grid: &Vec<Vec<u8>>, i: usize, j: usize, height: usize, width: usize) -> u32 {
    if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
        return 0;
    }

    let tree_height = grid[i][j];

    // Scan Right.
    let mut r_count = 0;
    for r in (j + 1)..width {
        r_count += 1;
        if grid[i][r] >= tree_height {
            break;
        }
    }

    // Scan Left.
    let mut l_count = 0;
    for r in (0..j).rev() {
        l_count += 1;
        if grid[i][r] >= tree_height {
            break;
        }
    }

    // Scan Down.
    let mut d_count = 0;
    for c in (i + 1)..height {
        d_count += 1;
        if grid[c][j] >= tree_height {
            break;
        }
    }

    // Scan Up.
    let mut u_count = 0;
    for c in (0..i).rev() {
        u_count += 1;
        if grid[c][j] >= tree_height {
            break;
        }
    }

    r_count * l_count * d_count * u_count
}

fn read_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| l.bytes().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
