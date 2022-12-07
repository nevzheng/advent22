use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use sscanf::sscanf;

pub fn part_one(input: &str) -> Option<u32> {
    let file_tree = parse_file_tree(input)?;

    let mut stack: PathBuf = PathBuf::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    subtree_sums(&file_tree, "/", &mut sizes, &mut stack);

    Some(sizes.into_values().filter(|v| v < &100_000).sum())
}

const GOAL: u32 = 30_000_000;
pub fn part_two(input: &str) -> Option<u32> {
    let file_tree = parse_file_tree(input)?;

    let mut stack: PathBuf = PathBuf::new();
    let mut sizes: HashMap<String, u32> = HashMap::new();
    subtree_sums(&file_tree, "/", &mut sizes, &mut stack);

    sizes
        .into_values()
        .filter(|d| d <= &GOAL)
        .min_by(|a, b| (GOAL - a).cmp(&(GOAL - b)))
}

fn subtree_sums(
    file_tree: &HashMap<String, HashSet<Node>>,
    cur: &str,
    sizes: &mut HashMap<String, u32>,
    stack: &mut PathBuf,
) -> u32 {
    // Save cur to stack.
    stack.push(cur);

    // Use Full Path to query file tree.
    let cur_full_path = stack.to_str().unwrap().to_owned();
    assert!(
        sizes.get(&cur_full_path).is_none(),
        "We can only visit each directory once.",
    );

    // Recurse and Sum Children.
    let subtree_sum = file_tree
        .get(&cur_full_path)
        .unwrap()
        .iter()
        .map(|e| match e {
            Node::Directory(dir_name) => subtree_sums(file_tree, dir_name, sizes, stack),
            Node::File(_, size) => *size,
        })
        .sum();

    // Populate subtree sum bottom up.
    assert!(
        sizes.insert(cur_full_path, subtree_sum).is_none(),
        "Paths must be unique"
    );
    // Pop current dir off stack.
    stack.pop();
    subtree_sum
}

fn parse_file_tree(input: &str) -> Option<HashMap<String, HashSet<Node>>> {
    let mut directories: HashMap<String, HashSet<Node>> = HashMap::new();
    let mut path: PathBuf = PathBuf::new();
    let mut cur_dir: String = String::new();
    let mut ls_dir: String = String::new();
    for line in input.lines() {
        // A Line is A Command Or a Node
        if let Some(cmd) = parse_command(line) {
            handle_commands(cmd, &mut cur_dir, &mut path, &mut ls_dir);
        } else {
            // Add the Node to the current `ls_dir`
            let node = parse_node(line)?;
            directories
                .entry(ls_dir.to_owned())
                .or_default()
                .insert(node);
        }
    }
    Some(directories)
}

fn handle_commands(cmd: Command, cur_dir: &mut String, stack: &mut PathBuf, ls_dir: &mut String) {
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

fn handle_cd(dir_name: Option<String>, cur_dir: &mut String, stack: &mut PathBuf) {
    if let Some(dir) = dir_name {
        // Save on stack for back tracking.
        stack.push(&dir);
        *cur_dir = stack.to_str().unwrap().to_owned();
    } else {
        // None means `cd ..`, pop & set last directory on stack.
        stack.pop();
        *cur_dir = stack.to_str().unwrap().to_owned();
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
        assert_eq!(part_two(&input), Some(24933642));
    }
}
