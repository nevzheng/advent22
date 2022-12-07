use std::collections::{HashMap, HashSet};

use sscanf::sscanf;

pub fn part_one(input: &str) -> Option<u32> {
    let file_tree = parse_file_tree(input)?;

    let mut sizes: HashMap<String, u32> = HashMap::new();
    subtree_sums(&file_tree, "/", &mut sizes);
    Some(sizes.into_values().filter(|v| v < &100_000).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn subtree_sums(
    file_tree: &HashMap<String, HashSet<Node>>,
    cur: &str,
    sizes: &mut HashMap<String, u32>,
) -> u32 {
    // if let Some(&subtree_sum) = sizes.get(cur) {
    if let Some(_) = sizes.get(cur) {
        panic!("I think I found a loop.");
        // return subtree_sum;
    }

    dbg!(file_tree.get(cur).unwrap());

    let subtree_sum = file_tree
        .get(cur)
        .unwrap()
        .iter()
        .map(|e| match e {
            Node::Directory(dir_name) => subtree_sums(file_tree, dir_name, sizes),
            Node::File(_, size) => *size,
        })
        .sum();
    // Populate subtree sum bottom up.
    assert!(sizes.insert(cur.to_owned(), subtree_sum).is_none());
    subtree_sum
}

fn parse_file_tree(input: &str) -> Option<HashMap<String, HashSet<Node>>> {
    let mut directories: HashMap<String, HashSet<Node>> = HashMap::new();
    let mut stack: Vec<String> = Vec::new();
    let mut cur_dir: String = String::new();
    let mut ls_dir: String = String::new();
    for line in input.lines() {
        // A Line is A Command Or a Node
        if let Some(cmd) = parse_command(line) {
            handle_commands(cmd, &mut cur_dir, &mut stack, &mut ls_dir);
        } else {
            // Add the Node to the current `ls_dir`
            let node = parse_node(line)?;
            directories
                .entry(ls_dir.to_owned())
                .or_default()
                .insert(node);
        }
    }
    dbg!(&directories);
    Some(directories)
}

fn handle_commands(
    cmd: Command,
    cur_dir: &mut String,
    stack: &mut Vec<String>,
    ls_dir: &mut String,
) {
    match cmd {
        Command::Cd(dir_name) => handle_cd(dir_name, cur_dir, stack),
        Command::Ls(dir_name) => handle_ls(dir_name, cur_dir, ls_dir),
    }
}

fn handle_ls(dir_name: Option<String>, cur_dir: &String, ls_dir: &mut String) {
    // If None, assume `cur_dir`.
    let dir = dir_name.unwrap_or_else(|| cur_dir.to_owned());
    // Set ls_dir to be written to in later iterations.
    *ls_dir = dir;
}

fn handle_cd(dir_name: Option<String>, cur_dir: &mut String, stack: &mut Vec<String>) {
    if let Some(dir) = dir_name {
        // Save on stack for back tracking.
        stack.push(dir.to_owned());
        *cur_dir = dir;
    } else {
        // None means `cd ..`, pop & set last directory on stack.
        *cur_dir = stack.pop().unwrap()
    }
}

fn parse_command(line: &str) -> Option<Command> {
    if !line.starts_with('$') {
        return None;
    }

    if line == "$ ls" {
        Some(Command::Ls(None))
    } else if line == "$ cd .." {
        Some(Command::Cd(None))
    } else if let Ok(dir_name) = sscanf!(line, "$ cd {str}") {
        Some(Command::Cd(Some(dir_name.to_owned())))
    } else {
        let dir_name = sscanf::scanf!(line, "$ ls {str}").ok()?;
        Some(Command::Ls(Some(dir_name.to_owned())))
    }
}

fn parse_node(line: &str) -> Option<Node> {
    if line.starts_with('$') {
        return None;
    }

    if let Ok((size, filename)) = sscanf!(line, "{u32} {str}") {
        Some(Node::File(filename.to_owned(), size))
    } else {
        let dir_name = sscanf!(line, "dir {str}").ok()?;
        Some(Node::Directory(dir_name.to_owned()))
    }
}

#[derive(Debug)]
enum Command {
    Cd(Option<String>),
    Ls(Option<String>),
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum Node {
    Directory(String),
    File(String, u32),
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
