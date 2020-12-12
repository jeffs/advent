pub enum Relative {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum Cardinal {
    East,
    North,
    West,
    South,
}

impl Cardinal {
    fn from_usize(index: usize) -> Cardinal {
        use Cardinal::*;
        const CARDINALS: [Cardinal; 4] = [East, North, West, South];
        CARDINALS[index % CARDINALS.len()]
    }

    pub fn turn(self, rel: Relative, degrees: usize) -> Cardinal {
        let index = self as usize;
        let rot = degrees / 90;
        match rel {
            Relative::Left => Cardinal::from_usize(index + rot),
            Relative::Right => Cardinal::from_usize(index + 4 - rot),
        }
    }
}
