use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use sscanf::sscanf;

pub fn part_one(input: &str) -> Option<usize> {
    let mut items: HashMap<usize, usize> = HashMap::new();
    let mut monkeys = parse_monkeys(input, &mut items);
    monkey_biz(&mut monkeys, &mut items, 20, true, None)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut items: HashMap<usize, usize> = HashMap::new();
    let mut monkeys = parse_monkeys(input, &mut items);

    // (a mod kn) mod n = a mod n for any integer k.
    // so instead of storing `a` we store `a mod kn`
    // where k = the product of all of the other checks
    let factor = monkeys
        .values()
        .map(|m| m.divisor)
        .reduce(|x, y| x * y)
        .unwrap();
    monkey_biz(&mut monkeys, &mut items, 10_000, false, Some(factor))
}

fn monkey_biz(
    monkeys: &mut HashMap<usize, Monkey>,
    items: &mut HashMap<usize, usize>,
    rounds: usize,
    divide_after_inspect: bool,
    factor: Option<usize>,
) -> Option<usize> {
    dbg!(&monkeys);
    for round_id in 1..=rounds {
        println!("Round {}", round_id);
        // Monkeys must go in order.
        for monkey_id in 0..monkeys.len() {
            // Updates to be made to current monkey and target monkey
            // let mut inspections = 0;
            let mut transfers: Vec<(usize, usize, usize)> = vec![];

            let monkey = monkeys.get(&monkey_id).unwrap();
            // let monkey = monkeys.get_mut(&(i as usize)).unwrap();

            for item_id in &monkey.items {
                // Monkey picks up item.
                // inspections += 1;
                let worry_level = items.get_mut(item_id).unwrap();

                // Apply Operation.
                let mut new_wl = match monkey.operation {
                    Operation::Add(x) => *worry_level + x,
                    Operation::Multiply(x) => *worry_level * x,
                    Operation::Square => *worry_level * *worry_level,
                    _ => panic!("invalid operation"),
                };
                if let Some(f) = factor {
                    new_wl %= f;
                }
                *worry_level = new_wl;

                // you react
                if divide_after_inspect {
                    *worry_level /= 3;
                }

                // Tests
                let transfer_tgt = if *worry_level % monkey.divisor == 0 {
                    monkey.true_throw_to
                } else {
                    monkey.false_throw_to
                };
                transfers.push((*item_id, monkey.id, transfer_tgt));
            }

            for (item_id, from, to) in transfers {
                monkeys.entry(from).and_modify(|m| {
                    m.inspections += 1;
                    m.items.remove(&item_id);
                });
                monkeys.entry(to).and_modify(|m| {
                    m.items.insert(item_id);
                });
            }
        }

        for monkey_id in 0..monkeys.len() {
            let monkey = monkeys.get(&monkey_id).unwrap();
            print!("{}: ", monkey.id);
            for v in &monkey.items {
                print!("{}, ", items[v])
            }
            println!()
        }
    }
    for monkey_id in 0..monkeys.len() {
        let monkey = monkeys.get(&monkey_id).unwrap();
        println!("{}: {}", monkey.id, monkey.inspections);
    }
    monkeys
        .values()
        .map(|m| m.inspections)
        .sorted_by(|x, y| y.cmp(x))
        .take(2)
        .reduce(|x, y| x * y)
}

fn parse_monkeys(input: &str, items: &mut HashMap<usize, usize>) -> HashMap<usize, Monkey> {
    let mut item_id: usize = 0;
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    for chunk in input.split("\n\n") {
        dbg!(&chunk);
        let mut monkey = Monkey::default();
        for (id, line) in chunk.lines().enumerate() {
            if id == 0 {
                let id = sscanf!(line, "Monkey {usize}:").unwrap();
                monkey.id = id;
            } else if id == 1 {
                let (_, worry_levels) = line.split_once(": ").unwrap();
                let worry_levels: Vec<usize> = worry_levels
                    .split(", ")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                for wl in worry_levels {
                    // Create a new item w/ wl
                    items.insert(item_id, wl);
                    // give the monkey a reference to the item
                    monkey.items.insert(item_id);
                    // increment item_id so each item gets a unique id.
                    item_id += 1;
                }
            } else if id == 2 {
                dbg!(&line);

                let operation: Operation = if line == "  Operation: new = old * old" {
                    Operation::Square
                } else {
                    let (c, arg) = sscanf!(line, "  Operation: new = old {char} {usize}").unwrap();
                    match c {
                        '+' => Operation::Add(arg),
                        '*' => Operation::Multiply(arg),
                        _ => panic!("unexpected op"),
                    }
                };
                monkey.operation = operation;
            } else if id == 3 {
                let divisor = sscanf!(line, "  Test: divisible by {usize}").unwrap();
                monkey.divisor = divisor;
            } else if id == 4 {
                let true_target = sscanf!(line, "    If true: throw to monkey {usize}").unwrap();
                monkey.true_throw_to = true_target;
            } else if id == 5 {
                let false_target = sscanf!(line, "    If false: throw to monkey {usize}").unwrap();
                monkey.false_throw_to = false_target;
            } else {
                panic!("unexpected number of lines in monkey chunk");
            }
        }

        monkeys.insert(monkey.id, monkey);
    }
    monkeys
}

#[derive(Debug)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
    None,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::None
    }
}

#[derive(Debug, Default)]
struct Monkey {
    id: usize,
    items: HashSet<usize>,
    divisor: usize,
    operation: Operation,
    true_throw_to: usize,
    false_throw_to: usize,
    inspections: usize,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
