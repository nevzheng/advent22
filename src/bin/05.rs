use sscanf::sscanf;
use std::{collections::HashMap};

pub fn part_one(input: &str) -> Option<String> {
    let (initial_state_str, instructions_str) = input.split_once("\n\n")?;

    let mut state = parse_initial_state(initial_state_str);
    let instructions = parse_instructions(instructions_str);

    for ins in instructions {
        for _ in 0..ins.count {
            let c = state.get_mut(&ins.src).unwrap().pop().unwrap();
            state.get_mut(&ins.dst).unwrap().push(c);
        }
    }

    let mut answer = String::new();
    for i in 0..state.len() {
        let stk_id: u32 = i as u32 + 1;
        let c = state.get(&stk_id).unwrap().last().unwrap();
        answer.push(*c);
    }

    println!("Part One: {}", &answer);
    Some(answer)
}

#[derive(Debug)]
struct Move {
    count: u32,
    src: u32,
    dst: u32,
}

pub fn part_two(input: &str) -> Option<String> {
    let (initial_state_str, instructions_str) = input.split_once("\n\n")?;

    let mut state = parse_initial_state(initial_state_str);
    let instructions = parse_instructions(instructions_str);

    for ins in instructions {
        let src = state.get_mut(&ins.src).unwrap();
        let at = src.len() - ins.count as usize;

        let sp = src.split_off(at);
        state.get_mut(&ins.dst).unwrap().extend(sp);
    }

    let mut answer = String::new();
    for i in 0..state.len() {
        let stk_id: u32 = i as u32 + 1;
        let c = state.get(&stk_id).unwrap().last().unwrap();
        answer.push(*c);
    }

    println!("Part One: {}", &answer);
    Some(answer)
}

fn parse_move(s: &str) -> Move {
    let (count, src, dst) = sscanf!(s, "move {u32} from {u32} to {u32}").unwrap();
    Move { count, src, dst }
}

fn parse_instructions(s: &str) -> Vec<Move> {
    let mut v = Vec::new();
    for l in s.lines() {
        v.push(parse_move(l));
    }
    v
}

fn parse_initial_state(s: &str) -> HashMap<u32, Vec<char>> {
    let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();
    let lines = s.lines();

    for l in lines {
        for idx in 0..9 {
            // 1, 5, 9, 13, 17, 21, 25, 29, 33
            let c_idx = 4 * idx + 1;
            // 1 indexed
            let stk_id = idx + 1;

            // Any OOB are skipped.
            if let Some(c) = l.chars().nth(c_idx) {
                // Skip empty entries.
                if !c.is_whitespace() && !c.is_numeric() {
                    stacks.entry(stk_id as u32).or_default().push(c);
                }
            }
        }
    }

    for v in stacks.values_mut() {
        v.reverse()
    }

    stacks
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    println!("Part One: {}", part_one(input).unwrap());
    println!("Part Two: {}", part_two(input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}