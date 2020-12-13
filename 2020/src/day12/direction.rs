#[derive(Clone, Copy, Debug)]
pub enum Cardinal {
    East,
    North,
    West,
    South,
}

impl Cardinal {
    fn from_isize(index: isize) -> Cardinal {
        use Cardinal::*;
        const CARDINALS: [Cardinal; 4] = [East, North, West, South];
        CARDINALS[((index % 4 + 4) % 4) as usize]
    }

    pub fn turn(self, degrees: isize) -> Cardinal {
        Cardinal::from_isize(self as isize + degrees / 90)
    }
}
