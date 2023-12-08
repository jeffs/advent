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

pub struct Node {
    pub name: String,
    left: String,
    right: String,
}

impl Node {
    pub fn next(&self, direction: Direction) -> &str {
        match direction {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

pub struct NodeMap {
    pub directions: Vec<Direction>,
    pub nodes: Vec<Node>,
    pub indexes: HashMap<String, usize>,
}

impl NodeMap {
    pub fn from_str(s: &str) -> NodeMap {
        let (first, rest) = s.split_once("\n\n").expect("the second line to be blank");
        let directions: Vec<Direction> = first.bytes().map(Direction::from_ascii).collect();
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
        NodeMap {
            directions,
            nodes,
            indexes,
        }
    }
}
