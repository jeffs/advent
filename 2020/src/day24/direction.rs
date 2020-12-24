pub enum HexDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl HexDirection {
    /// HexDirections can't implement std::str::FromStr because it depends on
    /// the lifetime of the supplied string reference.  FromStr really wants
    /// its result to be independent of the lifetime of the original string.
    pub fn parse_line(line: &str) -> HexDirections {
        HexDirections { line }
    }
}

pub enum SquareDirection {
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

pub struct HexDirections<'a> {
    line: &'a str,
}

impl Iterator for HexDirections<'_> {
    type Item = HexDirection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.starts_with('e') {
            self.line = &self.line[1..];
            Some(HexDirection::East)
        } else if self.line.starts_with("ne") {
            self.line = &self.line[2..];
            Some(HexDirection::NorthEast)
        } else if self.line.starts_with("nw") {
            self.line = &self.line[2..];
            Some(HexDirection::NorthWest)
        } else if self.line.starts_with('w') {
            self.line = &self.line[1..];
            Some(HexDirection::West)
        } else if self.line.starts_with("sw") {
            self.line = &self.line[2..];
            Some(HexDirection::SouthWest)
        } else if self.line.starts_with("se") {
            self.line = &self.line[2..];
            Some(HexDirection::SouthEast)
        } else {
            None
        }
    }
}
