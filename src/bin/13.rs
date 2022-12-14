use std::cmp::Ordering;

use serde_json::{json, Result, Value};

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        read_input(input)
            .iter()
            .map(|(l, r)| cmp(l, r))
            .enumerate()
            .filter(|(_, p)| p.is_some() && matches!(p.unwrap(), Ordering::Less))
            .map(|(i, _)| i + 1)
            .sum::<usize>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets: Vec<Value> = Vec::new();
    read_input(input).into_iter().for_each(|(a, b)| {
        packets.push(a);
        packets.push(b);
    });

    packets.extend([json!([[2]]), json!([[6]])]);

    packets.sort_by(|a, b| cmp(a, b).unwrap());
    let dp1 = packets.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let dp2 = packets.iter().position(|p| *p == json!([[6]])).unwrap() + 1;

    Some((dp1 * dp2) as u32)
}

fn cmp(l: &Value, r: &Value) -> Option<Ordering> {
    if let (Some(a), Some(b)) = (l.as_u64(), r.as_u64()) {
        // Both Nums
        match a.cmp(&b) {
            Ordering::Equal => None,
            order => Some(order),
        }
    } else if let (Some(va), Some(vb)) = (l.as_array(), r.as_array()) {
        // we used recursion and found arrays are different lengths.
        if va.is_empty() || vb.is_empty() {
            match va.len().cmp(&vb.len()) {
                Ordering::Equal => None,
                order => Some(order),
            }
        } else if let Some(v) = cmp(&va[0], &vb[0]) {
            // Comparing the first elements yielded an ordering
            // could be any combo of value types.
            Some(v)
        } else {
            // Comparing the two produce a value so we continue.
            cmp(&json!(va[1..]), &json!(vb[1..]))
        }
        // Both Arrays
    } else if let (Some(a), Some(vb)) = (l.as_u64(), r.as_array()) {
        // a is u64, vb is array: Convert a => va
        cmp(&json!(vec![a]), &json!(vb))
    } else if let (Some(va), Some(b)) = (l.as_array(), r.as_u64()) {
        // va is array, b is u64, conver b => vb
        cmp(&json!(va), &json!(vec![b]))
    } else {
        Some(Ordering::Greater)
    }
}

fn read_input(input: &str) -> Vec<(Value, Value)> {
    input
        .split("\n\n")
        .map(|chunk| {
            let (l, r) = chunk.split_once('\n').unwrap();
            let l: Value = serde_json::from_str(l).unwrap();
            let r: Value = serde_json::from_str(r).unwrap();
            (l, r)
        })
        .collect::<Vec<(Value, Value)>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
