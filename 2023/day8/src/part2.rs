use crate::node::{Node, NodeMap};

fn is_start(name: &str) -> bool {
    name.ends_with('A')
}

fn is_final(node: &Node) -> bool {
    node.name.ends_with('Z')
}

pub fn distance_from(map: &NodeMap, node: &str) -> usize {
    let mut index = map.indexes[node];
    for (count, &direction) in map.directions.iter().cycle().enumerate() {
        let node = &map.nodes[index];
        if is_final(node) {
            return count;
        }
        let next = node.next(direction);
        index = map.indexes[next];
    }
    unreachable!()
}

/// Returns the Greatest Common Divisor of a and b.
fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

/// Returns the Least Common Multiple of a and b.
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve(text: &str) -> usize {
    let map = NodeMap::from_str(text);
    map.nodes
        .iter()
        .map(|node| &node.name)
        .filter(|name| is_start(name))
        .map(|start| distance_from(&map, start))
        .filter(|&distance| distance != 0)
        .fold(1, lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample2.txt")), 6);
    }
}
