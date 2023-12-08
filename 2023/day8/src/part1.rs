use crate::node::NodeMap;

pub fn solve(text: &str) -> usize {
    NodeMap::from_str(text).distance("AAA", |name| name == "ZZZ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample1.txt")), 2);
    }
}
