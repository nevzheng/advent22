use sscanf::sscanf;

pub fn part_one(input: &str) -> Option<i32> {
    let mut reg = 1i32;
    let mut cycle = 1;

    let mut acc = 0;

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
            check_cycle(cycle, &reg, &mut acc);
        } else if let Ok(arg) = sscanf!(line, "addx {i32}") {
            cycle += 1;
            check_cycle(cycle, &reg, &mut acc);
            cycle += 1;
            reg += arg;
            check_cycle(cycle, &reg, &mut acc);
        }
    }

    Some(acc)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut reg = 1i32;
    let mut cycle = 1;
    let mut crt = vec![vec!['.'; 40]; 6];

    for line in input.lines() {
        if line == "noop" {
            cycle += 1;
        } else if let Ok(arg) = sscanf!(line, "addx {i32}") {
            cycle += 1;
            draw_crt(cycle, reg, &mut crt);
            reg += arg;
            cycle += 1;
        }
        draw_crt(cycle, reg, &mut crt);
    }

    print_crt(&crt);
    None
}

fn draw_crt(cycle: i32, reg: i32, crt: &mut [Vec<char>]) {
    let row = ((cycle - 1) / 40) % 6;
    let col = (cycle - 1) % 40;
    if reg >= col - 1 && reg <= col + 1 {
        crt[row as usize][col as usize] = '#';
    }
}

fn print_crt(crt: &[Vec<char>]) {
    for line in crt {
        for c in line {
            print!("{}", c);
        }
        println!()
    }
}

fn check_cycle(cycle: i32, reg: &i32, acc: &mut i32) {
    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        *acc += cycle * (reg)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
