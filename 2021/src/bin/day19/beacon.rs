use advent2021::ParseError;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub type Offset = (i32, i32, i32); // dx, dy, dz

fn next_coord<'a, I>(coords: &mut I) -> Result<i32, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    Ok(coords
        .next()
        .ok_or_else(|| ParseError::new("expected coordinate"))?
        .parse()?)
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Beacon(pub i32, pub i32, pub i32); // x, y, z

impl Beacon {
    /// Rotates this Beacon a quarter turn counterclockwise around the Y axis.
    pub fn orbit_y_left(&self) -> Beacon {
        let Beacon(x, y, z) = *self;
        Beacon(-z, y, x)
    }

    /// Rotates this Beacon a quarter turn clockwise around the Y axis.
    pub fn orbit_y_right(&self) -> Beacon {
        let Beacon(x, y, z) = *self;
        Beacon(z, y, -x)
    }

    /// Rotates this Beacon a quarter turn clockwise around the Y axis.
    pub fn orbit_z_right(&self) -> Beacon {
        let Beacon(x, y, z) = *self;
        Beacon(y, -x, z)
    }
}

impl FromStr for Beacon {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = &mut s.split(',');
        let beacon = Beacon(
            next_coord(coords)?,
            next_coord(coords)?,
            next_coord(coords)?,
        );
        if let Some(extra) = coords.next() {
            let what = format!("expected end of line, not {}", extra);
            return Err(ParseError::new(what));
        }
        Ok(beacon)
    }
}

impl Add<Offset> for Beacon {
    type Output = Beacon;

    fn add(self, other: Offset) -> Self::Output {
        Beacon(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Beacon {
    type Output = Offset;

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}
