use std::collections::HashMap;

struct Node {
    name: String,
    left: String,
    right: String,
}

pub fn solve(text: &str) -> usize {
    let (first, rest) = text
        .split_once("\n\n")
        .expect("the second line to be blank");

    let nodes: Vec<Node> = rest
        .lines()
        .map(|line| Node {
            name: line[..3].to_string(),
            left: line[7..10].to_string(),
            right: line[12..15].to_string(),
        })
        .collect();

    let indexes: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(index, node)| (node.name.clone(), index))
        .collect();

    let mut index = indexes["AAA"];
    for (count, lr) in first.bytes().cycle().enumerate() {
        let node = &nodes[index];
        if node.name == "ZZZ" {
            return count;
        }
        let next = match lr {
            b'L' => &node.left,
            b'R' => &node.right,
            _ => unreachable!(),
        };
        index = indexes[next];
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 2);
    }
}
