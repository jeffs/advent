use crate::node::NodeMap;

/// Returns the Greatest Common Divisor of a and b.
fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        (a, b) = (b % a, a);
    }
    b
}

/// Returns the Least Common Multiple of a and b.
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn solve(text: &str) -> usize {
    let map = NodeMap::from_str(text);
    map.nodes
        .iter()
        .map(|node| node.name)
        .filter(|name| name.ends_with('A'))
        .map(|start| map.distance(start, |name| name.ends_with('Z')))
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
