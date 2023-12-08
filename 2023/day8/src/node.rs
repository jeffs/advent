use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_ascii(c: u8) -> Direction {
        match c {
            b'L' => Direction::Left,
            b'R' => Direction::Right,
            _ => unreachable!(),
        }
    }
}

pub struct Node<'a> {
    pub name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl Node<'_> {
    pub fn next(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

pub struct NodeMap<'a> {
    directions: Vec<Direction>,
    pub nodes: Vec<Node<'a>>,
    indexes: HashMap<&'a str, usize>,
}

impl NodeMap<'_> {
    pub fn from_str(s: &str) -> NodeMap {
        let (first, rest) = s.split_once("\n\n").expect("the second line to be blank");
        let directions: Vec<Direction> = first.bytes().map(Direction::from_ascii).collect();
        let nodes: Vec<Node> = rest
            .lines()
            .map(|line| Node {
                name: &line[..3],
                left: &line[7..10],
                right: &line[12..15],
            })
            .collect();
        let indexes: HashMap<&str, usize> = nodes
            .iter()
            .enumerate()
            .map(|(index, node)| (node.name, index))
            .collect();
        NodeMap {
            directions,
            nodes,
            indexes,
        }
    }

    /// Returns the path length from the specified start node to node for which
    /// the specified is_final predicate returns true.
    pub fn distance<F: Fn(&str) -> bool>(&self, node: &str, is_final: F) -> usize {
        let mut index = self.indexes[node];
        for (count, &direction) in self.directions.iter().cycle().enumerate() {
            let node = &self.nodes[index];
            if is_final(node.name) {
                return count;
            }
            let next = node.next(direction);
            index = self.indexes[next];
        }
        unreachable!()
    }
}
