use crate::image::Image;
use advent2021::ParseError;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

const TABLE_SIZE: usize = 512;

/// Image enhancement algorithm.
pub struct Algorithm {
    table: [bool; TABLE_SIZE], // false for dark (.), true for light (#)
}

impl Algorithm {
    #[allow(clippy::identity_op)]
    #[rustfmt::skip]
    pub fn enhance(&self, old: &Image) -> Image {
        let mut special = HashSet::new();
        let bg_index = if old.background() { TABLE_SIZE - 1 } else { 0 };
        let default = self.table[bg_index];
        for i in (old.min_i() - 1)..(old.max_i() + 2) {
            for j in (old.min_j() - 1)..(old.max_j() + 2) {
                let key = (old.at(i - 1, j - 1) as usize) << 8
                        | (old.at(i - 1, j - 0) as usize) << 7
                        | (old.at(i - 1, j + 1) as usize) << 6
                        | (old.at(i - 0, j - 1) as usize) << 5
                        | (old.at(i - 0, j - 0) as usize) << 4
                        | (old.at(i - 0, j + 1) as usize) << 3
                        | (old.at(i + 1, j - 1) as usize) << 2
                        | (old.at(i + 1, j - 0) as usize) << 1
                        | (old.at(i + 1, j + 1) as usize) << 0;
                if self.table[key] != default {
                    special.insert((i, j));
                }
            }
        }
        Image::new(default, special)
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for b in self.table {
            let c = if b { '#' } else { '.' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

impl FromStr for Algorithm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if TABLE_SIZE != s.len() {
            let what = format!("want {} bytes; got {}", TABLE_SIZE, s.len());
            return Err(ParseError::new(what));
        }
        let mut table = [false; TABLE_SIZE];
        let mut bytes = s.bytes();
        table.fill_with(|| bytes.next() == Some(b'#'));
        Ok(Algorithm { table })
    }
}
