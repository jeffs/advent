pub enum HexDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

impl HexDirection {
    pub fn all() -> HexDirections {
        HexDirections {
            next: Some(HexDirection::East),
        }
    }

    /// HexDirections can't implement std::str::FromStr because it depends on
    /// the lifetime of the supplied string reference.  FromStr really wants
    /// its result to be independent of the lifetime of the original string.
    pub fn parse_line(line: &str) -> ParseLine {
        ParseLine { line }
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

pub struct HexDirections {
    next: Option<HexDirection>,
}

impl Iterator for HexDirections {
    type Item = HexDirection;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(HexDirection::East) => self.next.replace(HexDirection::NorthEast),
            Some(HexDirection::NorthEast) => self.next.replace(HexDirection::NorthWest),
            Some(HexDirection::NorthWest) => self.next.replace(HexDirection::West),
            Some(HexDirection::West) => self.next.replace(HexDirection::SouthWest),
            Some(HexDirection::SouthWest) => self.next.replace(HexDirection::SouthEast),
            Some(HexDirection::SouthEast) => self.next.take(),
            None => None,
        }
    }
}

pub struct ParseLine<'a> {
    line: &'a str,
}

impl Iterator for ParseLine<'_> {
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
