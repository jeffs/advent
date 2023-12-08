use crate::node::NodeMap;

pub fn distance_from(map: &NodeMap, node: &str) -> usize {
    let mut index = map.indexes[node];
    for (count, &direction) in map.directions.iter().cycle().enumerate() {
        let node = &map.nodes[index];
        if node.name == "ZZZ" {
            return count;
        }
        let next = node.next(direction);
        index = map.indexes[next];
    }
    unreachable!()
}

pub fn solve(text: &str) -> usize {
    distance_from(&NodeMap::from_str(text), "AAA")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample1.txt")), 2);
    }
}
